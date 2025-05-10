use std::fs;

use rustc_hir::{HirId, Node};
use rustc_hir::def_id::DefId;
use rustc_middle::ty::TyCtxt;
use rustc_middle::ty::ParamEnvAnd;
use std::collections::{HashMap, HashSet};
use rustc_hir::intravisit;
use rustc_middle::hir::nested_filter;
use rustc_middle::ty::DefIdTree;
use rustc_span::Span;

use petgraph::graphmap::DiGraphMap;
use petgraph::dot::{Dot, Config};

use crate::utils::*;

macro_rules! skip_generated_code {
    ($span: expr) => {
        if $span.from_expansion() || $span.is_dummy() {
            return;
        }
    };
}

// Backup self.cur_fn, set cur_fn to id, continue to walk the AST by executing
// $walk, then restore self.cur_fn.
macro_rules! push_walk_pop {
    ($this: expr, $id: expr, $walk: expr) => {{
        let prev_fn = $this.cur_fn;
        $this.cur_fn = Some($id);
        $walk;
        $this.cur_fn = prev_fn;
    }};
}

#[derive(Hash, PartialEq, Eq, Debug)]
pub struct Call {
    // the call expression
    pub call_expr: HirId,
    pub call_expr_span: Span,
    // possible enclosing function
    pub caller: Option<DefId>,
    pub caller_span: Option<Span>,
    // call target
    pub callee: DefId,
    pub callee_span: Span,
}

pub struct CallgraphVisitor<'tcx> {
    // type context
    pub tcx: TyCtxt<'tcx>,

    // free functions
    pub functions: HashMap<DefId, Span>,
    // global variables
    pub globals: HashMap<DefId, HashSet<Span>>,
    // trait method declarations without default implementation
    pub method_decls: HashSet<DefId>,
    // map decls to impls
    pub method_impls: HashMap<DefId, Vec<DefId>>,

    // static calls
    pub static_calls: HashSet<Call>,
    // dynamic calls
    pub dynamic_calls: HashSet<Call>,

    pub graph: DiGraphMap<DefId, i32>,

    pub imports: HashSet<Span>,

    // tracks the current function we're in during AST walk
    cur_fn: Option<DefId>,
}

impl<'tcx> CallgraphVisitor<'tcx> {
    pub fn new(tcx: &TyCtxt<'tcx>) -> CallgraphVisitor<'tcx> {
        CallgraphVisitor {
            tcx: *tcx,
            functions: HashMap::new(),
            globals: HashMap::new(),
            method_decls: HashSet::new(),
            method_impls: HashMap::new(),
            static_calls: HashSet::new(),
            dynamic_calls: HashSet::new(),
            graph: DiGraphMap::new(),
            imports: HashSet::new(),
            cur_fn: None,
        }
    }

    pub fn print_calls(&self) {
        let dot_output = format!("{:?}", Dot::new(&self.graph));
        fs::write("callgraph.dot", dot_output);
    }
}

impl<'tcx> intravisit::Visitor<'tcx> for CallgraphVisitor<'tcx> {

    type NestedFilter = nested_filter::OnlyBodies;

    fn nested_visit_map(&mut self) -> Self::Map {
        self.tcx.hir()
    }

    fn visit_expr(&mut self, expr: &'tcx rustc_hir::Expr) {
        skip_generated_code!(expr.span);

        let hir_id = expr.hir_id;
        match expr.kind {
            rustc_hir::ExprKind::Call(
                    rustc_hir::Expr{
                        kind: rustc_hir::ExprKind::Path(ref qpath),
                        ..
                    }, _) => {
                if let rustc_hir::QPath::Resolved(_, p) = qpath {
                    if let rustc_hir::def::Res::Def(_, def_id) = p.res {
                        self.static_calls.insert(Call {
                            call_expr: hir_id,
                            call_expr_span: expr.span,
                            caller: self.cur_fn,
                            caller_span: None,
                            callee: def_id,
                            callee_span: p.span,
                        });
                        if let Some(cur_fn_def_id) = self.cur_fn {
                            if def_id.is_local() {
                                self.graph.add_edge(cur_fn_def_id, def_id, 1);
                            }
                        }
                    }
                }
            },            
            rustc_hir::ExprKind::MethodCall(_, _, _) => {
                let o_def_id = hir_id.owner;
                let typeck_tables = self.tcx.typeck(o_def_id);
                let substs = typeck_tables.node_substs(hir_id); // Substitutions
                let method_id = typeck_tables.type_dependent_def_id(hir_id).expect("fail");
                let param_env = self.tcx.param_env(method_id);
                if let Ok(Some(inst)) =
                    self.tcx.resolve_instance(ParamEnvAnd{param_env, value: (method_id, substs)})
                {
                    let res_def_id = inst.def_id();
                    let node = self.tcx.hir().get_if_local(res_def_id);
                    match node {
                        Some(Node::TraitItem(rustc_hir::TraitItem{span, ..})) => {
                            // dynamic calls resolve only to the trait method decl
                            self.dynamic_calls.insert(Call {
                                call_expr: hir_id,
                                call_expr_span: expr.span,
                                caller: self.cur_fn,
                                caller_span: None,
                                callee: res_def_id,
                                callee_span: *span,
                            });
                            if let Some(cur_fn_def_id) = self.cur_fn {
                                if res_def_id.is_local() {
                                    self.graph.add_edge(cur_fn_def_id, res_def_id, 1);
                                }
                            }
                        }
                        Some(Node::ImplItem(rustc_hir::ImplItem{span, ..})) |
                                Some(Node::Item(rustc_hir::Item{span, ..})) |
                                Some(Node::ForeignItem(rustc_hir::ForeignItem{span, ..})) => {
                            // calls for which the receiver's type can be resolved
                            self.static_calls.insert(Call {
                                call_expr: hir_id,
                                call_expr_span: expr.span,
                                caller: self.cur_fn,
                                caller_span: None,
                                callee: res_def_id,
                                callee_span: *span,
                            });
                            if let Some(cur_fn_def_id) = self.cur_fn {
                                if res_def_id.is_local() {
                                    self.graph.add_edge(cur_fn_def_id, res_def_id, 1);
                                }
                            }
                        },
                        None => (),
                        _ => todo!(),
                    };
                }
            },
            rustc_hir::ExprKind::Path(ref qpath) => {
                if let rustc_hir::QPath::Resolved(_, p) = qpath {
                    if let rustc_hir::def::Res::Def(_, def_id) = p.res {
                        if let Some(local_def_id) = def_id.as_local() {
                            // We got the node corresponding to the definition
                            if let Some(node) = self.tcx.hir().find_by_def_id(local_def_id) {
                                if let rustc_hir::Node::Item(
                                        rustc_hir::Item{kind, span, ..}) = node {
                                    match kind {
                                        rustc_hir::ItemKind::Fn(..) => {
                                            // Do nothing - we don't want functions
                                        },
                                        _ => {
                                            // If the definition is outside the current function
                                            if let Some(cur_fn_def_id) = self.cur_fn {
                                                if !self.tcx.is_descendant_of(def_id, cur_fn_def_id) {
                                                    // Add it to the globals for this function
                                                    self.globals.entry(cur_fn_def_id).or_default().insert(*span);
                                                }
                                            }
                                        },
                                    }
                                }
                            }
                        }
                    }
                }
            },
            rustc_hir::ExprKind::Type(ref expr, ty) => {
                if let Some(cur_fn_def_id) = self.cur_fn {
                    self.globals.entry(cur_fn_def_id).or_default().insert(ty.span);
                }
            },
            _ => {}
        }
        // traverse
        intravisit::walk_expr(self, expr);
    }

    fn visit_item(&mut self, item: &'tcx rustc_hir::Item) {
        skip_generated_code!(item.span);

        let hir_id = item.hir_id();
        if let rustc_hir::ItemKind::Fn(_, _, _) = item.kind {
            let def_id = hir_id.owner.to_def_id();
            self.functions.insert(def_id, item.span);
            if !self.graph.contains_node(def_id) {
                self.graph.add_node(def_id);
            }
            push_walk_pop!(self, def_id, intravisit::walk_item(self, item));

            return;
        }
        if let rustc_hir::ItemKind::Use(..) = item.kind {
            self.imports.insert(item.span);
            return;
        }
        // traverse
        intravisit::walk_item(self, item)
    }

    fn visit_trait_item(&mut self, ti: &'tcx rustc_hir::TraitItem) {
        skip_generated_code!(ti.span); // TODO ?do we want this

        let hir_id = ti.hir_id();
        let def_id = hir_id.owner.to_def_id();

        match ti.kind {
            rustc_hir::TraitItemKind::Fn(_, rustc_hir::TraitFn::Required(_)) => {
                // a method declaration
                self.method_decls.insert(def_id);
                self.method_impls.insert(def_id, vec![]);
            }
            rustc_hir::TraitItemKind::Fn(_, rustc_hir::TraitFn::Provided(_)) => {
                // a method decl and def
                self.method_decls.insert(def_id);
                self.functions.insert(def_id, ti.span);
                if !self.graph.contains_node(def_id) {
                    self.graph.add_node(def_id);
                }
                self.method_impls.entry(def_id).or_default().push(def_id);

                push_walk_pop!(self, def_id, intravisit::walk_trait_item(self, ti));

                return;
            }
            _ => {}
        }

        // traverse
        intravisit::walk_trait_item(self, ti)
    }

    // self.tcx.hir().hir_to_pretty_string(ty.hir_id)

    fn visit_impl_item(&mut self, ii: &'tcx rustc_hir::ImplItem) {
        skip_generated_code!(ii.span);

        let hir_id = ii.hir_id();
        let def_id = hir_id.owner.to_def_id();

        if let rustc_hir::ImplItemKind::Fn(..) = ii.kind {
            self.functions.insert(def_id, ii.span);
            if !self.graph.contains_node(def_id) {
                self.graph.add_node(def_id);
            }

            // store link to decl
            let mut decl_id = None;
            if let Some(impl_id) = self.tcx.impl_of_method(def_id) {
                if let Some(Node::Item(item)) = self.tcx.hir().get_if_local(impl_id) {
                    if let rustc_hir::ItemKind::Impl(..) = item.kind {
                        // the next one filters methods that are just associated
                        // and do not belong to a struct
                        if let Some(trait_def_id) = self.tcx.trait_id_of_impl(impl_id) {
                            let item = self.tcx
                                .associated_items(trait_def_id)
                                .filter_by_name_unhygienic(ii.ident.name)
                                .next(); // There should ideally be only one item matching the name
                            if let Some(item) = item {
                                decl_id = Some(item.def_id);
                            };
                        }
                    }
                }
            }

            if let Some(decl_def_id) = decl_id {
                self.method_impls
                    .entry(decl_def_id)
                    .or_default()
                    .push(def_id);
            }

            push_walk_pop!(self, def_id, intravisit::walk_impl_item(self, ii));

            return;
        }

        // traverse
        intravisit::walk_impl_item(self, ii)
    }
}