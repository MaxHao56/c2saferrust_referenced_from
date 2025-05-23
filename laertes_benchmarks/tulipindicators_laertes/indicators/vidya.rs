
extern "C" {
    
    fn sqrt(_: std::os::raw::c_double) -> std::os::raw::c_double;
    
    fn __assert_rtn(_: * const std::os::raw::c_char, _: * const std::os::raw::c_char,
                    _: std::os::raw::c_int, _: * const std::os::raw::c_char) -> !;
}
/*
 * Tulip Indicators
 * https://tulipindicators.org/
 * Copyright (c) 2010-2017 Tulip Charts LLC
 * Lewis Van Winkle (LV@tulipcharts.org)
 *
 * This file is part of Tulip Indicators.
 *
 * Tulip Indicators is free software: you can redistribute it and/or modify it
 * under the terms of the GNU Lesser General Public License as published by the
 * Free Software Foundation, either version 3 of the License, or (at your
 * option) any later version.
 *
 * Tulip Indicators is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
 * FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License
 * for more details.
 *
 * You should have received a copy of the GNU Lesser General Public License
 * along with Tulip Indicators.  If not, see <http://www.gnu.org/licenses/>.
 *
 */
#[no_mangle]
pub unsafe extern "C" fn ti_vidya_start(mut options: * const std::os::raw::c_double)
 -> std::os::raw::c_int {
    return *options.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int -
               2 as
                   std::os::raw::c_int; /* In some conditions it works out that we take the sqrt(-0.0), which gives NaN.
                              That implies that k should be zero. */
}
#[no_mangle]
pub unsafe extern "C" fn ti_vidya(mut size: std::os::raw::c_int,
                                  mut inputs: * const * const std::os::raw::c_double,
                                  mut options: * const std::os::raw::c_double,
                                  mut outputs: * const * mut std::os::raw::c_double)
 -> std::os::raw::c_int {
    let mut input: * const f64 =
        *inputs.offset(0 as std::os::raw::c_int as isize);
    let short_period: i32 =
        *options.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int;
    let long_period: i32 =
        *options.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int;
    let alpha: f64 = *options.offset(2 as std::os::raw::c_int as isize);
    let mut output: * mut f64 =
        *outputs.offset(0 as std::os::raw::c_int as isize);
    let short_div: f64 = 1.0f64 / short_period as std::os::raw::c_double;
    let long_div: f64 = 1.0f64 / long_period as std::os::raw::c_double;
    if short_period < 1 as std::os::raw::c_int { return 1 as std::os::raw::c_int }
    if long_period < short_period { return 1 as std::os::raw::c_int }
    if long_period < 2 as std::os::raw::c_int { return 1 as std::os::raw::c_int }
    if alpha < 0.0f64 || alpha > 1.0f64 { return 1 as std::os::raw::c_int }
    if size <= ti_vidya_start(options) { return 0 as std::os::raw::c_int }
    let mut short_sum: f64 = 0 as std::os::raw::c_int as std::os::raw::c_double;
    let mut short_sum2: f64 = 0 as std::os::raw::c_int as std::os::raw::c_double;
    let mut long_sum: f64 = 0 as std::os::raw::c_int as std::os::raw::c_double;
    let mut long_sum2: f64 = 0 as std::os::raw::c_int as std::os::raw::c_double;
    let mut i: i32 = 0;
    i = 0 as std::os::raw::c_int;
    while i < long_period {
        long_sum += *input.offset(i as isize);
        long_sum2 += *input.offset(i as isize) * *input.offset(i as isize);
        if i >= long_period - short_period {
            short_sum += *input.offset(i as isize);
            short_sum2 +=
                *input.offset(i as isize) * *input.offset(i as isize)
        }
        i += 1
    }
    let mut val: f64 =
        *input.offset((long_period - 2 as std::os::raw::c_int) as isize);
    let mut fresh0 = output;
    output = output.offset(1);
    *fresh0 = val;
    if (long_period - 1 as std::os::raw::c_int) < size {
        let mut short_stddev: f64 =
            sqrt(short_sum2 * short_div -
                     short_sum * short_div * (short_sum * short_div));
        let mut long_stddev: f64 =
            sqrt(long_sum2 * long_div -
                     long_sum * long_div * (long_sum * long_div));
        let mut k: f64 = short_stddev / long_stddev;
        if k != k { k = 0 as std::os::raw::c_int as std::os::raw::c_double }
        k *= alpha;
        val =
            (*input.offset((long_period - 1 as std::os::raw::c_int) as isize) - val) *
                k + val;
        let mut fresh1 = output;
        output = output.offset(1);
        *fresh1 = val
    }
    i = long_period;
    while i < size {
        long_sum += *input.offset(i as isize);
        long_sum2 += *input.offset(i as isize) * *input.offset(i as isize);
        short_sum += *input.offset(i as isize);
        short_sum2 += *input.offset(i as isize) * *input.offset(i as isize);
        long_sum -= *input.offset((i - long_period) as isize);
        long_sum2 -=
            *input.offset((i - long_period) as isize) *
                *input.offset((i - long_period) as isize);
        short_sum -= *input.offset((i - short_period) as isize);
        short_sum2 -=
            *input.offset((i - short_period) as isize) *
                *input.offset((i - short_period) as isize);
        let mut short_stddev_0: f64 =
            sqrt(short_sum2 * short_div -
                     short_sum * short_div * (short_sum * short_div));
        let mut long_stddev_0: f64 =
            sqrt(long_sum2 * long_div -
                     long_sum * long_div * (long_sum * long_div));
        let mut k_0: f64 = short_stddev_0 / long_stddev_0;
        if k_0 != k_0 { k_0 = 0 as std::os::raw::c_int as std::os::raw::c_double }
        k_0 *= alpha;
        val = (*input.offset(i as isize) - val) * k_0 + val;
        let mut fresh2 = output;
        output = output.offset(1);
        *fresh2 = val;
        i += 1
    }
    if !(output.offset_from(*outputs.offset(0 as std::os::raw::c_int as
                                                         isize)) as
             std::os::raw::c_long == (size - ti_vidya_start(options)) as std::os::raw::c_long)
           as std::os::raw::c_int as std::os::raw::c_long != 0 {
        __assert_rtn((*core::intrinsics::transmute::<&'_ [u8; 9], &'_ [i8; 9]>(b"ti_vidya\x00")).as_ptr(),
                     b"indicators/vidya.c\x00" as *const u8 as
                         *const std::os::raw::c_char, 106 as std::os::raw::c_int,
                     b"output - outputs[0] == size - ti_vidya_start(options)\x00"
                         as *const u8 as *const std::os::raw::c_char);
    } else { };
    return 0 as std::os::raw::c_int;
}
use crate::laertes_rt::*;
