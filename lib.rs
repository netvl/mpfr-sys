#![allow(non_camel_case_types, raw_pointer_deriving)]

extern crate libc;
extern crate "gmp-sys" as gmp;
use libc::size_t;
use gmp::{mp_limb_t, gmp_randstate_t, mpf_ptr, mpz_ptr, mpz_srcptr, mpq_srcptr, mpf_srcptr};

/* automatically generated by rust-bindgen */

pub type mpfr_void = ::libc::c_void;
pub type mpfr_int = ::libc::c_int;
pub type mpfr_uint = ::libc::c_uint;
pub type mpfr_long = ::libc::c_long;
pub type mpfr_ulong = ::libc::c_ulong;
pub type mpfr_size_t = size_t;
pub type Enum_Unnamed1 = ::libc::c_int;
pub const MPFR_RNDN: ::libc::c_int = 0;
pub const MPFR_RNDZ: ::libc::c_int = 1;
pub const MPFR_RNDU: ::libc::c_int = 2;
pub const MPFR_RNDD: ::libc::c_int = 3;
pub const MPFR_RNDA: ::libc::c_int = 4;
pub const MPFR_RNDF: ::libc::c_int = 5;
pub const MPFR_RNDNA: ::libc::c_int = -1;
pub type mpfr_rnd_t = Enum_Unnamed1;
pub type mpfr_prec_t = ::libc::c_long;
pub type mpfr_uprec_t = ::libc::c_ulong;
pub type mpfr_sign_t = ::libc::c_int;
pub type mpfr_exp_t = ::libc::c_long;
pub type mpfr_uexp_t = ::libc::c_ulong;
#[repr(C)]
#[deriving(Copy)]
pub struct Struct_Unnamed2 {
    pub _mpfr_prec: mpfr_prec_t,
    pub _mpfr_sign: mpfr_sign_t,
    pub _mpfr_exp: mpfr_exp_t,
    pub _mpfr_d: *mut mp_limb_t,
}
impl ::std::default::Default for Struct_Unnamed2 {
    fn default() -> Struct_Unnamed2 { unsafe { ::std::mem::zeroed() } }
}
pub type __mpfr_struct = Struct_Unnamed2;
pub type mpfr_t = [__mpfr_struct; 1u];
pub type mpfr_ptr = *mut __mpfr_struct;
pub type mpfr_srcptr = *const __mpfr_struct;
pub type Enum_Unnamed3 = ::libc::c_uint;
pub const MPFR_NAN_KIND: ::libc::c_uint = 0;
pub const MPFR_INF_KIND: ::libc::c_uint = 1;
pub const MPFR_ZERO_KIND: ::libc::c_uint = 2;
pub const MPFR_REGULAR_KIND: ::libc::c_uint = 3;
pub type mpfr_kind_t = Enum_Unnamed3;
extern "C" {
    pub fn mpfr_get_version() -> *const ::libc::c_char;
    pub fn mpfr_get_patches() -> *const ::libc::c_char;
    pub fn mpfr_buildopt_tls_p() -> ::libc::c_int;
    pub fn mpfr_buildopt_decimal_p() -> ::libc::c_int;
    pub fn mpfr_buildopt_gmpinternals_p() -> ::libc::c_int;
    pub fn mpfr_buildopt_tune_case() -> *const ::libc::c_char;
    pub fn mpfr_get_emin() -> mpfr_exp_t;
    pub fn mpfr_set_emin(arg1: mpfr_exp_t) -> ::libc::c_int;
    pub fn mpfr_get_emin_min() -> mpfr_exp_t;
    pub fn mpfr_get_emin_max() -> mpfr_exp_t;
    pub fn mpfr_get_emax() -> mpfr_exp_t;
    pub fn mpfr_set_emax(arg1: mpfr_exp_t) -> ::libc::c_int;
    pub fn mpfr_get_emax_min() -> mpfr_exp_t;
    pub fn mpfr_get_emax_max() -> mpfr_exp_t;
    pub fn mpfr_set_default_rounding_mode(arg1: mpfr_rnd_t);
    pub fn mpfr_get_default_rounding_mode() -> mpfr_rnd_t;
    pub fn mpfr_print_rnd_mode(arg1: mpfr_rnd_t) -> *const ::libc::c_char;
    pub fn mpfr_clear_flags();
    pub fn mpfr_clear_underflow();
    pub fn mpfr_clear_overflow();
    pub fn mpfr_clear_divby0();
    pub fn mpfr_clear_nanflag();
    pub fn mpfr_clear_inexflag();
    pub fn mpfr_clear_erangeflag();
    pub fn mpfr_set_underflow();
    pub fn mpfr_set_overflow();
    pub fn mpfr_set_divby0();
    pub fn mpfr_set_nanflag();
    pub fn mpfr_set_inexflag();
    pub fn mpfr_set_erangeflag();
    pub fn mpfr_underflow_p() -> ::libc::c_int;
    pub fn mpfr_overflow_p() -> ::libc::c_int;
    pub fn mpfr_divby0_p() -> ::libc::c_int;
    pub fn mpfr_nanflag_p() -> ::libc::c_int;
    pub fn mpfr_inexflag_p() -> ::libc::c_int;
    pub fn mpfr_erangeflag_p() -> ::libc::c_int;
    pub fn mpfr_check_range(arg1: mpfr_ptr, arg2: ::libc::c_int,
                            arg3: mpfr_rnd_t) -> ::libc::c_int;
    pub fn mpfr_init2(arg1: mpfr_ptr, arg2: mpfr_prec_t);
    pub fn mpfr_init(arg1: mpfr_ptr);
    pub fn mpfr_clear(arg1: mpfr_ptr);
    pub fn mpfr_inits2(arg1: mpfr_prec_t, arg2: mpfr_ptr, ...);
    pub fn mpfr_inits(arg1: mpfr_ptr, ...);
    pub fn mpfr_clears(arg1: mpfr_ptr, ...);
    pub fn mpfr_prec_round(arg1: mpfr_ptr, arg2: mpfr_prec_t,
                           arg3: mpfr_rnd_t) -> ::libc::c_int;
    pub fn mpfr_can_round(arg1: mpfr_srcptr, arg2: mpfr_exp_t,
                          arg3: mpfr_rnd_t, arg4: mpfr_rnd_t,
                          arg5: mpfr_prec_t) -> ::libc::c_int;
    pub fn mpfr_min_prec(arg1: mpfr_srcptr) -> mpfr_prec_t;
    pub fn mpfr_get_exp(arg1: mpfr_srcptr) -> mpfr_exp_t;
    pub fn mpfr_set_exp(arg1: mpfr_ptr, arg2: mpfr_exp_t) -> ::libc::c_int;
    pub fn mpfr_get_prec(arg1: mpfr_srcptr) -> mpfr_prec_t;
    pub fn mpfr_set_prec(arg1: mpfr_ptr, arg2: mpfr_prec_t);
    pub fn mpfr_set_prec_raw(arg1: mpfr_ptr, arg2: mpfr_prec_t);
    pub fn mpfr_set_default_prec(arg1: mpfr_prec_t);
    pub fn mpfr_get_default_prec() -> mpfr_prec_t;
    pub fn mpfr_set_d(arg1: mpfr_ptr, arg2: ::libc::c_double,
                      arg3: mpfr_rnd_t) -> ::libc::c_int;
    pub fn mpfr_set_flt(arg1: mpfr_ptr, arg2: ::libc::c_float,
                        arg3: mpfr_rnd_t) -> ::libc::c_int;
    pub fn mpfr_set_ld(arg1: mpfr_ptr, arg2: ::libc::c_double,
                       arg3: mpfr_rnd_t) -> ::libc::c_int;
    pub fn mpfr_set_z(arg1: mpfr_ptr, arg2: mpz_srcptr, arg3: mpfr_rnd_t)
     -> ::libc::c_int;
    pub fn mpfr_set_z_2exp(arg1: mpfr_ptr, arg2: mpz_srcptr, arg3: mpfr_exp_t,
                           arg4: mpfr_rnd_t) -> ::libc::c_int;
    pub fn mpfr_set_nan(arg1: mpfr_ptr);
    pub fn mpfr_set_inf(arg1: mpfr_ptr, arg2: ::libc::c_int);
    pub fn mpfr_set_zero(arg1: mpfr_ptr, arg2: ::libc::c_int);
    pub fn mpfr_set_f(arg1: mpfr_ptr, arg2: mpf_srcptr, arg3: mpfr_rnd_t)
     -> ::libc::c_int;
    pub fn mpfr_get_f(arg1: mpf_ptr, arg2: mpfr_srcptr, arg3: mpfr_rnd_t)
     -> ::libc::c_int;
    pub fn mpfr_set_si(arg1: mpfr_ptr, arg2: ::libc::c_long, arg3: mpfr_rnd_t)
     -> ::libc::c_int;
    pub fn mpfr_set_ui(arg1: mpfr_ptr, arg2: ::libc::c_ulong,
                       arg3: mpfr_rnd_t) -> ::libc::c_int;
    pub fn mpfr_set_si_2exp(arg1: mpfr_ptr, arg2: ::libc::c_long,
                            arg3: mpfr_exp_t, arg4: mpfr_rnd_t)
     -> ::libc::c_int;
    pub fn mpfr_set_ui_2exp(arg1: mpfr_ptr, arg2: ::libc::c_ulong,
                            arg3: mpfr_exp_t, arg4: mpfr_rnd_t)
     -> ::libc::c_int;
    pub fn mpfr_set_q(arg1: mpfr_ptr, arg2: mpq_srcptr, arg3: mpfr_rnd_t)
     -> ::libc::c_int;
    pub fn mpfr_set_str(arg1: mpfr_ptr, arg2: *const ::libc::c_char,
                        arg3: ::libc::c_int, arg4: mpfr_rnd_t)
     -> ::libc::c_int;
    pub fn mpfr_init_set_str(arg1: mpfr_ptr, arg2: *const ::libc::c_char,
                             arg3: ::libc::c_int, arg4: mpfr_rnd_t)
     -> ::libc::c_int;
    pub fn mpfr_set4(arg1: mpfr_ptr, arg2: mpfr_srcptr, arg3: mpfr_rnd_t,
                     arg4: ::libc::c_int) -> ::libc::c_int;
    pub fn mpfr_abs(arg1: mpfr_ptr, arg2: mpfr_srcptr, arg3: mpfr_rnd_t)
     -> ::libc::c_int;
    pub fn mpfr_set(arg1: mpfr_ptr, arg2: mpfr_srcptr, arg3: mpfr_rnd_t)
     -> ::libc::c_int;
    pub fn mpfr_neg(arg1: mpfr_ptr, arg2: mpfr_srcptr, arg3: mpfr_rnd_t)
     -> ::libc::c_int;
    pub fn mpfr_signbit(arg1: mpfr_srcptr) -> ::libc::c_int;
    pub fn mpfr_setsign(arg1: mpfr_ptr, arg2: mpfr_srcptr,
                        arg3: ::libc::c_int, arg4: mpfr_rnd_t)
     -> ::libc::c_int;
    pub fn mpfr_copysign(arg1: mpfr_ptr, arg2: mpfr_srcptr, arg3: mpfr_srcptr,
                         arg4: mpfr_rnd_t) -> ::libc::c_int;
    pub fn mpfr_get_z_2exp(arg1: mpz_ptr, arg2: mpfr_srcptr) -> mpfr_exp_t;
    pub fn mpfr_get_flt(arg1: mpfr_srcptr, arg2: mpfr_rnd_t)
     -> ::libc::c_float;
    pub fn mpfr_get_d(arg1: mpfr_srcptr, arg2: mpfr_rnd_t)
     -> ::libc::c_double;
    pub fn mpfr_get_ld(arg1: mpfr_srcptr, arg2: mpfr_rnd_t)
     -> ::libc::c_double;
    pub fn mpfr_get_d1(arg1: mpfr_srcptr) -> ::libc::c_double;
    pub fn mpfr_get_d_2exp(arg1: *mut ::libc::c_long, arg2: mpfr_srcptr,
                           arg3: mpfr_rnd_t) -> ::libc::c_double;
    pub fn mpfr_get_ld_2exp(arg1: *mut ::libc::c_long, arg2: mpfr_srcptr,
                            arg3: mpfr_rnd_t) -> ::libc::c_double;
    pub fn mpfr_frexp(arg1: *mut mpfr_exp_t, arg2: mpfr_ptr,
                      arg3: mpfr_srcptr, arg4: mpfr_rnd_t) -> ::libc::c_int;
    pub fn mpfr_get_si(arg1: mpfr_srcptr, arg2: mpfr_rnd_t) -> ::libc::c_long;
    pub fn mpfr_get_ui(arg1: mpfr_srcptr, arg2: mpfr_rnd_t)
     -> ::libc::c_ulong;
    pub fn mpfr_get_str(arg1: *mut ::libc::c_char, arg2: *mut mpfr_exp_t,
                        arg3: ::libc::c_int, arg4: size_t, arg5: mpfr_srcptr,
                        arg6: mpfr_rnd_t) -> *mut ::libc::c_char;
    pub fn mpfr_get_z(z: mpz_ptr, f: mpfr_srcptr, arg1: mpfr_rnd_t)
     -> ::libc::c_int;
    pub fn mpfr_free_str(arg1: *mut ::libc::c_char);
    pub fn mpfr_urandom(arg1: mpfr_ptr, arg2: gmp_randstate_t,
                        arg3: mpfr_rnd_t) -> ::libc::c_int;
    pub fn mpfr_grandom(arg1: mpfr_ptr, arg2: mpfr_ptr, arg3: gmp_randstate_t,
                        arg4: mpfr_rnd_t) -> ::libc::c_int;
    pub fn mpfr_urandomb(arg1: mpfr_ptr, arg2: gmp_randstate_t)
     -> ::libc::c_int;
    pub fn mpfr_nextabove(arg1: mpfr_ptr);
    pub fn mpfr_nextbelow(arg1: mpfr_ptr);
    pub fn mpfr_nexttoward(arg1: mpfr_ptr, arg2: mpfr_srcptr);
    pub fn mpfr_printf(arg1: *const ::libc::c_char, ...) -> ::libc::c_int;
    pub fn mpfr_asprintf(arg1: *mut *mut ::libc::c_char,
                         arg2: *const ::libc::c_char, ...) -> ::libc::c_int;
    pub fn mpfr_sprintf(arg1: *mut ::libc::c_char,
                        arg2: *const ::libc::c_char, ...) -> ::libc::c_int;
    pub fn mpfr_snprintf(arg1: *mut ::libc::c_char, arg2: size_t,
                         arg3: *const ::libc::c_char, ...) -> ::libc::c_int;
    pub fn mpfr_pow(arg1: mpfr_ptr, arg2: mpfr_srcptr, arg3: mpfr_srcptr,
                    arg4: mpfr_rnd_t) -> ::libc::c_int;
    pub fn mpfr_pow_si(arg1: mpfr_ptr, arg2: mpfr_srcptr,
                       arg3: ::libc::c_long, arg4: mpfr_rnd_t)
     -> ::libc::c_int;
    pub fn mpfr_pow_ui(arg1: mpfr_ptr, arg2: mpfr_srcptr,
                       arg3: ::libc::c_ulong, arg4: mpfr_rnd_t)
     -> ::libc::c_int;
    pub fn mpfr_ui_pow_ui(arg1: mpfr_ptr, arg2: ::libc::c_ulong,
                          arg3: ::libc::c_ulong, arg4: mpfr_rnd_t)
     -> ::libc::c_int;
    pub fn mpfr_ui_pow(arg1: mpfr_ptr, arg2: ::libc::c_ulong,
                       arg3: mpfr_srcptr, arg4: mpfr_rnd_t) -> ::libc::c_int;
    pub fn mpfr_pow_z(arg1: mpfr_ptr, arg2: mpfr_srcptr, arg3: mpz_srcptr,
                      arg4: mpfr_rnd_t) -> ::libc::c_int;
    pub fn mpfr_sqrt(arg1: mpfr_ptr, arg2: mpfr_srcptr, arg3: mpfr_rnd_t)
     -> ::libc::c_int;
    pub fn mpfr_sqrt_ui(arg1: mpfr_ptr, arg2: ::libc::c_ulong,
                        arg3: mpfr_rnd_t) -> ::libc::c_int;
    pub fn mpfr_rec_sqrt(arg1: mpfr_ptr, arg2: mpfr_srcptr, arg3: mpfr_rnd_t)
     -> ::libc::c_int;
    pub fn mpfr_add(arg1: mpfr_ptr, arg2: mpfr_srcptr, arg3: mpfr_srcptr,
                    arg4: mpfr_rnd_t) -> ::libc::c_int;
    pub fn mpfr_sub(arg1: mpfr_ptr, arg2: mpfr_srcptr, arg3: mpfr_srcptr,
                    arg4: mpfr_rnd_t) -> ::libc::c_int;
    pub fn mpfr_mul(arg1: mpfr_ptr, arg2: mpfr_srcptr, arg3: mpfr_srcptr,
                    arg4: mpfr_rnd_t) -> ::libc::c_int;
    pub fn mpfr_div(arg1: mpfr_ptr, arg2: mpfr_srcptr, arg3: mpfr_srcptr,
                    arg4: mpfr_rnd_t) -> ::libc::c_int;
    pub fn mpfr_add_ui(arg1: mpfr_ptr, arg2: mpfr_srcptr,
                       arg3: ::libc::c_ulong, arg4: mpfr_rnd_t)
     -> ::libc::c_int;
    pub fn mpfr_sub_ui(arg1: mpfr_ptr, arg2: mpfr_srcptr,
                       arg3: ::libc::c_ulong, arg4: mpfr_rnd_t)
     -> ::libc::c_int;
    pub fn mpfr_ui_sub(arg1: mpfr_ptr, arg2: ::libc::c_ulong,
                       arg3: mpfr_srcptr, arg4: mpfr_rnd_t) -> ::libc::c_int;
    pub fn mpfr_mul_ui(arg1: mpfr_ptr, arg2: mpfr_srcptr,
                       arg3: ::libc::c_ulong, arg4: mpfr_rnd_t)
     -> ::libc::c_int;
    pub fn mpfr_div_ui(arg1: mpfr_ptr, arg2: mpfr_srcptr,
                       arg3: ::libc::c_ulong, arg4: mpfr_rnd_t)
     -> ::libc::c_int;
    pub fn mpfr_ui_div(arg1: mpfr_ptr, arg2: ::libc::c_ulong,
                       arg3: mpfr_srcptr, arg4: mpfr_rnd_t) -> ::libc::c_int;
    pub fn mpfr_add_si(arg1: mpfr_ptr, arg2: mpfr_srcptr,
                       arg3: ::libc::c_long, arg4: mpfr_rnd_t)
     -> ::libc::c_int;
    pub fn mpfr_sub_si(arg1: mpfr_ptr, arg2: mpfr_srcptr,
                       arg3: ::libc::c_long, arg4: mpfr_rnd_t)
     -> ::libc::c_int;
    pub fn mpfr_si_sub(arg1: mpfr_ptr, arg2: ::libc::c_long,
                       arg3: mpfr_srcptr, arg4: mpfr_rnd_t) -> ::libc::c_int;
    pub fn mpfr_mul_si(arg1: mpfr_ptr, arg2: mpfr_srcptr,
                       arg3: ::libc::c_long, arg4: mpfr_rnd_t)
     -> ::libc::c_int;
    pub fn mpfr_div_si(arg1: mpfr_ptr, arg2: mpfr_srcptr,
                       arg3: ::libc::c_long, arg4: mpfr_rnd_t)
     -> ::libc::c_int;
    pub fn mpfr_si_div(arg1: mpfr_ptr, arg2: ::libc::c_long,
                       arg3: mpfr_srcptr, arg4: mpfr_rnd_t) -> ::libc::c_int;
    pub fn mpfr_add_d(arg1: mpfr_ptr, arg2: mpfr_srcptr,
                      arg3: ::libc::c_double, arg4: mpfr_rnd_t)
     -> ::libc::c_int;
    pub fn mpfr_sub_d(arg1: mpfr_ptr, arg2: mpfr_srcptr,
                      arg3: ::libc::c_double, arg4: mpfr_rnd_t)
     -> ::libc::c_int;
    pub fn mpfr_d_sub(arg1: mpfr_ptr, arg2: ::libc::c_double,
                      arg3: mpfr_srcptr, arg4: mpfr_rnd_t) -> ::libc::c_int;
    pub fn mpfr_mul_d(arg1: mpfr_ptr, arg2: mpfr_srcptr,
                      arg3: ::libc::c_double, arg4: mpfr_rnd_t)
     -> ::libc::c_int;
    pub fn mpfr_div_d(arg1: mpfr_ptr, arg2: mpfr_srcptr,
                      arg3: ::libc::c_double, arg4: mpfr_rnd_t)
     -> ::libc::c_int;
    pub fn mpfr_d_div(arg1: mpfr_ptr, arg2: ::libc::c_double,
                      arg3: mpfr_srcptr, arg4: mpfr_rnd_t) -> ::libc::c_int;
    pub fn mpfr_sqr(arg1: mpfr_ptr, arg2: mpfr_srcptr, arg3: mpfr_rnd_t)
     -> ::libc::c_int;
    pub fn mpfr_const_pi(arg1: mpfr_ptr, arg2: mpfr_rnd_t) -> ::libc::c_int;
    pub fn mpfr_const_log2(arg1: mpfr_ptr, arg2: mpfr_rnd_t) -> ::libc::c_int;
    pub fn mpfr_const_euler(arg1: mpfr_ptr, arg2: mpfr_rnd_t)
     -> ::libc::c_int;
    pub fn mpfr_const_catalan(arg1: mpfr_ptr, arg2: mpfr_rnd_t)
     -> ::libc::c_int;
    pub fn mpfr_agm(arg1: mpfr_ptr, arg2: mpfr_srcptr, arg3: mpfr_srcptr,
                    arg4: mpfr_rnd_t) -> ::libc::c_int;
    pub fn mpfr_log(arg1: mpfr_ptr, arg2: mpfr_srcptr, arg3: mpfr_rnd_t)
     -> ::libc::c_int;
    pub fn mpfr_log2(arg1: mpfr_ptr, arg2: mpfr_srcptr, arg3: mpfr_rnd_t)
     -> ::libc::c_int;
    pub fn mpfr_log10(arg1: mpfr_ptr, arg2: mpfr_srcptr, arg3: mpfr_rnd_t)
     -> ::libc::c_int;
    pub fn mpfr_log1p(arg1: mpfr_ptr, arg2: mpfr_srcptr, arg3: mpfr_rnd_t)
     -> ::libc::c_int;
    pub fn mpfr_exp(arg1: mpfr_ptr, arg2: mpfr_srcptr, arg3: mpfr_rnd_t)
     -> ::libc::c_int;
    pub fn mpfr_exp2(arg1: mpfr_ptr, arg2: mpfr_srcptr, arg3: mpfr_rnd_t)
     -> ::libc::c_int;
    pub fn mpfr_exp10(arg1: mpfr_ptr, arg2: mpfr_srcptr, arg3: mpfr_rnd_t)
     -> ::libc::c_int;
    pub fn mpfr_expm1(arg1: mpfr_ptr, arg2: mpfr_srcptr, arg3: mpfr_rnd_t)
     -> ::libc::c_int;
    pub fn mpfr_eint(arg1: mpfr_ptr, arg2: mpfr_srcptr, arg3: mpfr_rnd_t)
     -> ::libc::c_int;
    pub fn mpfr_li2(arg1: mpfr_ptr, arg2: mpfr_srcptr, arg3: mpfr_rnd_t)
     -> ::libc::c_int;
    pub fn mpfr_cmp(arg1: mpfr_srcptr, arg2: mpfr_srcptr) -> ::libc::c_int;
    pub fn mpfr_cmp3(arg1: mpfr_srcptr, arg2: mpfr_srcptr,
                     arg3: ::libc::c_int) -> ::libc::c_int;
    pub fn mpfr_cmp_d(arg1: mpfr_srcptr, arg2: ::libc::c_double)
     -> ::libc::c_int;
    pub fn mpfr_cmp_ld(arg1: mpfr_srcptr, arg2: ::libc::c_double)
     -> ::libc::c_int;
    pub fn mpfr_cmpabs(arg1: mpfr_srcptr, arg2: mpfr_srcptr) -> ::libc::c_int;
    pub fn mpfr_cmp_ui(arg1: mpfr_srcptr, arg2: ::libc::c_ulong)
     -> ::libc::c_int;
    pub fn mpfr_cmp_si(arg1: mpfr_srcptr, arg2: ::libc::c_long)
     -> ::libc::c_int;
    pub fn mpfr_cmp_ui_2exp(arg1: mpfr_srcptr, arg2: ::libc::c_ulong,
                            arg3: mpfr_exp_t) -> ::libc::c_int;
    pub fn mpfr_cmp_si_2exp(arg1: mpfr_srcptr, arg2: ::libc::c_long,
                            arg3: mpfr_exp_t) -> ::libc::c_int;
    pub fn mpfr_reldiff(arg1: mpfr_ptr, arg2: mpfr_srcptr, arg3: mpfr_srcptr,
                        arg4: mpfr_rnd_t);
    pub fn mpfr_eq(arg1: mpfr_srcptr, arg2: mpfr_srcptr,
                   arg3: ::libc::c_ulong) -> ::libc::c_int;
    pub fn mpfr_sgn(arg1: mpfr_srcptr) -> ::libc::c_int;
    pub fn mpfr_mul_2exp(arg1: mpfr_ptr, arg2: mpfr_srcptr,
                         arg3: ::libc::c_ulong, arg4: mpfr_rnd_t)
     -> ::libc::c_int;
    pub fn mpfr_div_2exp(arg1: mpfr_ptr, arg2: mpfr_srcptr,
                         arg3: ::libc::c_ulong, arg4: mpfr_rnd_t)
     -> ::libc::c_int;
    pub fn mpfr_mul_2ui(arg1: mpfr_ptr, arg2: mpfr_srcptr,
                        arg3: ::libc::c_ulong, arg4: mpfr_rnd_t)
     -> ::libc::c_int;
    pub fn mpfr_div_2ui(arg1: mpfr_ptr, arg2: mpfr_srcptr,
                        arg3: ::libc::c_ulong, arg4: mpfr_rnd_t)
     -> ::libc::c_int;
    pub fn mpfr_mul_2si(arg1: mpfr_ptr, arg2: mpfr_srcptr,
                        arg3: ::libc::c_long, arg4: mpfr_rnd_t)
     -> ::libc::c_int;
    pub fn mpfr_div_2si(arg1: mpfr_ptr, arg2: mpfr_srcptr,
                        arg3: ::libc::c_long, arg4: mpfr_rnd_t)
     -> ::libc::c_int;
    pub fn mpfr_rint(arg1: mpfr_ptr, arg2: mpfr_srcptr, arg3: mpfr_rnd_t)
     -> ::libc::c_int;
    pub fn mpfr_round(arg1: mpfr_ptr, arg2: mpfr_srcptr) -> ::libc::c_int;
    pub fn mpfr_trunc(arg1: mpfr_ptr, arg2: mpfr_srcptr) -> ::libc::c_int;
    pub fn mpfr_ceil(arg1: mpfr_ptr, arg2: mpfr_srcptr) -> ::libc::c_int;
    pub fn mpfr_floor(arg1: mpfr_ptr, arg2: mpfr_srcptr) -> ::libc::c_int;
    pub fn mpfr_rint_round(arg1: mpfr_ptr, arg2: mpfr_srcptr,
                           arg3: mpfr_rnd_t) -> ::libc::c_int;
    pub fn mpfr_rint_trunc(arg1: mpfr_ptr, arg2: mpfr_srcptr,
                           arg3: mpfr_rnd_t) -> ::libc::c_int;
    pub fn mpfr_rint_ceil(arg1: mpfr_ptr, arg2: mpfr_srcptr, arg3: mpfr_rnd_t)
     -> ::libc::c_int;
    pub fn mpfr_rint_floor(arg1: mpfr_ptr, arg2: mpfr_srcptr,
                           arg3: mpfr_rnd_t) -> ::libc::c_int;
    pub fn mpfr_frac(arg1: mpfr_ptr, arg2: mpfr_srcptr, arg3: mpfr_rnd_t)
     -> ::libc::c_int;
    pub fn mpfr_modf(arg1: mpfr_ptr, arg2: mpfr_ptr, arg3: mpfr_srcptr,
                     arg4: mpfr_rnd_t) -> ::libc::c_int;
    pub fn mpfr_remquo(arg1: mpfr_ptr, arg2: *mut ::libc::c_long,
                       arg3: mpfr_srcptr, arg4: mpfr_srcptr, arg5: mpfr_rnd_t)
     -> ::libc::c_int;
    pub fn mpfr_remainder(arg1: mpfr_ptr, arg2: mpfr_srcptr,
                          arg3: mpfr_srcptr, arg4: mpfr_rnd_t)
     -> ::libc::c_int;
    pub fn mpfr_fmod(arg1: mpfr_ptr, arg2: mpfr_srcptr, arg3: mpfr_srcptr,
                     arg4: mpfr_rnd_t) -> ::libc::c_int;
    pub fn mpfr_fits_ulong_p(arg1: mpfr_srcptr, arg2: mpfr_rnd_t)
     -> ::libc::c_int;
    pub fn mpfr_fits_slong_p(arg1: mpfr_srcptr, arg2: mpfr_rnd_t)
     -> ::libc::c_int;
    pub fn mpfr_fits_uint_p(arg1: mpfr_srcptr, arg2: mpfr_rnd_t)
     -> ::libc::c_int;
    pub fn mpfr_fits_sint_p(arg1: mpfr_srcptr, arg2: mpfr_rnd_t)
     -> ::libc::c_int;
    pub fn mpfr_fits_ushort_p(arg1: mpfr_srcptr, arg2: mpfr_rnd_t)
     -> ::libc::c_int;
    pub fn mpfr_fits_sshort_p(arg1: mpfr_srcptr, arg2: mpfr_rnd_t)
     -> ::libc::c_int;
    pub fn mpfr_fits_uintmax_p(arg1: mpfr_srcptr, arg2: mpfr_rnd_t)
     -> ::libc::c_int;
    pub fn mpfr_fits_intmax_p(arg1: mpfr_srcptr, arg2: mpfr_rnd_t)
     -> ::libc::c_int;
    pub fn mpfr_extract(arg1: mpz_ptr, arg2: mpfr_srcptr,
                        arg3: ::libc::c_uint);
    pub fn mpfr_swap(arg1: mpfr_ptr, arg2: mpfr_ptr);
    pub fn mpfr_dump(arg1: mpfr_srcptr);
    pub fn mpfr_nan_p(arg1: mpfr_srcptr) -> ::libc::c_int;
    pub fn mpfr_inf_p(arg1: mpfr_srcptr) -> ::libc::c_int;
    pub fn mpfr_number_p(arg1: mpfr_srcptr) -> ::libc::c_int;
    pub fn mpfr_integer_p(arg1: mpfr_srcptr) -> ::libc::c_int;
    pub fn mpfr_zero_p(arg1: mpfr_srcptr) -> ::libc::c_int;
    pub fn mpfr_regular_p(arg1: mpfr_srcptr) -> ::libc::c_int;
    pub fn mpfr_greater_p(arg1: mpfr_srcptr, arg2: mpfr_srcptr)
     -> ::libc::c_int;
    pub fn mpfr_greaterequal_p(arg1: mpfr_srcptr, arg2: mpfr_srcptr)
     -> ::libc::c_int;
    pub fn mpfr_less_p(arg1: mpfr_srcptr, arg2: mpfr_srcptr) -> ::libc::c_int;
    pub fn mpfr_lessequal_p(arg1: mpfr_srcptr, arg2: mpfr_srcptr)
     -> ::libc::c_int;
    pub fn mpfr_lessgreater_p(arg1: mpfr_srcptr, arg2: mpfr_srcptr)
     -> ::libc::c_int;
    pub fn mpfr_equal_p(arg1: mpfr_srcptr, arg2: mpfr_srcptr)
     -> ::libc::c_int;
    pub fn mpfr_unordered_p(arg1: mpfr_srcptr, arg2: mpfr_srcptr)
     -> ::libc::c_int;
    pub fn mpfr_atanh(arg1: mpfr_ptr, arg2: mpfr_srcptr, arg3: mpfr_rnd_t)
     -> ::libc::c_int;
    pub fn mpfr_acosh(arg1: mpfr_ptr, arg2: mpfr_srcptr, arg3: mpfr_rnd_t)
     -> ::libc::c_int;
    pub fn mpfr_asinh(arg1: mpfr_ptr, arg2: mpfr_srcptr, arg3: mpfr_rnd_t)
     -> ::libc::c_int;
    pub fn mpfr_cosh(arg1: mpfr_ptr, arg2: mpfr_srcptr, arg3: mpfr_rnd_t)
     -> ::libc::c_int;
    pub fn mpfr_sinh(arg1: mpfr_ptr, arg2: mpfr_srcptr, arg3: mpfr_rnd_t)
     -> ::libc::c_int;
    pub fn mpfr_tanh(arg1: mpfr_ptr, arg2: mpfr_srcptr, arg3: mpfr_rnd_t)
     -> ::libc::c_int;
    pub fn mpfr_sinh_cosh(arg1: mpfr_ptr, arg2: mpfr_ptr, arg3: mpfr_srcptr,
                          arg4: mpfr_rnd_t) -> ::libc::c_int;
    pub fn mpfr_sech(arg1: mpfr_ptr, arg2: mpfr_srcptr, arg3: mpfr_rnd_t)
     -> ::libc::c_int;
    pub fn mpfr_csch(arg1: mpfr_ptr, arg2: mpfr_srcptr, arg3: mpfr_rnd_t)
     -> ::libc::c_int;
    pub fn mpfr_coth(arg1: mpfr_ptr, arg2: mpfr_srcptr, arg3: mpfr_rnd_t)
     -> ::libc::c_int;
    pub fn mpfr_acos(arg1: mpfr_ptr, arg2: mpfr_srcptr, arg3: mpfr_rnd_t)
     -> ::libc::c_int;
    pub fn mpfr_asin(arg1: mpfr_ptr, arg2: mpfr_srcptr, arg3: mpfr_rnd_t)
     -> ::libc::c_int;
    pub fn mpfr_atan(arg1: mpfr_ptr, arg2: mpfr_srcptr, arg3: mpfr_rnd_t)
     -> ::libc::c_int;
    pub fn mpfr_sin(arg1: mpfr_ptr, arg2: mpfr_srcptr, arg3: mpfr_rnd_t)
     -> ::libc::c_int;
    pub fn mpfr_sin_cos(arg1: mpfr_ptr, arg2: mpfr_ptr, arg3: mpfr_srcptr,
                        arg4: mpfr_rnd_t) -> ::libc::c_int;
    pub fn mpfr_cos(arg1: mpfr_ptr, arg2: mpfr_srcptr, arg3: mpfr_rnd_t)
     -> ::libc::c_int;
    pub fn mpfr_tan(arg1: mpfr_ptr, arg2: mpfr_srcptr, arg3: mpfr_rnd_t)
     -> ::libc::c_int;
    pub fn mpfr_atan2(arg1: mpfr_ptr, arg2: mpfr_srcptr, arg3: mpfr_srcptr,
                      arg4: mpfr_rnd_t) -> ::libc::c_int;
    pub fn mpfr_sec(arg1: mpfr_ptr, arg2: mpfr_srcptr, arg3: mpfr_rnd_t)
     -> ::libc::c_int;
    pub fn mpfr_csc(arg1: mpfr_ptr, arg2: mpfr_srcptr, arg3: mpfr_rnd_t)
     -> ::libc::c_int;
    pub fn mpfr_cot(arg1: mpfr_ptr, arg2: mpfr_srcptr, arg3: mpfr_rnd_t)
     -> ::libc::c_int;
    pub fn mpfr_hypot(arg1: mpfr_ptr, arg2: mpfr_srcptr, arg3: mpfr_srcptr,
                      arg4: mpfr_rnd_t) -> ::libc::c_int;
    pub fn mpfr_erf(arg1: mpfr_ptr, arg2: mpfr_srcptr, arg3: mpfr_rnd_t)
     -> ::libc::c_int;
    pub fn mpfr_erfc(arg1: mpfr_ptr, arg2: mpfr_srcptr, arg3: mpfr_rnd_t)
     -> ::libc::c_int;
    pub fn mpfr_cbrt(arg1: mpfr_ptr, arg2: mpfr_srcptr, arg3: mpfr_rnd_t)
     -> ::libc::c_int;
    pub fn mpfr_root(arg1: mpfr_ptr, arg2: mpfr_srcptr, arg3: ::libc::c_ulong,
                     arg4: mpfr_rnd_t) -> ::libc::c_int;
    pub fn mpfr_gamma(arg1: mpfr_ptr, arg2: mpfr_srcptr, arg3: mpfr_rnd_t)
     -> ::libc::c_int;
    pub fn mpfr_lngamma(arg1: mpfr_ptr, arg2: mpfr_srcptr, arg3: mpfr_rnd_t)
     -> ::libc::c_int;
    pub fn mpfr_lgamma(arg1: mpfr_ptr, arg2: *mut ::libc::c_int,
                       arg3: mpfr_srcptr, arg4: mpfr_rnd_t) -> ::libc::c_int;
    pub fn mpfr_digamma(arg1: mpfr_ptr, arg2: mpfr_srcptr, arg3: mpfr_rnd_t)
     -> ::libc::c_int;
    pub fn mpfr_zeta(arg1: mpfr_ptr, arg2: mpfr_srcptr, arg3: mpfr_rnd_t)
     -> ::libc::c_int;
    pub fn mpfr_zeta_ui(arg1: mpfr_ptr, arg2: ::libc::c_ulong,
                        arg3: mpfr_rnd_t) -> ::libc::c_int;
    pub fn mpfr_fac_ui(arg1: mpfr_ptr, arg2: ::libc::c_ulong,
                       arg3: mpfr_rnd_t) -> ::libc::c_int;
    pub fn mpfr_j0(arg1: mpfr_ptr, arg2: mpfr_srcptr, arg3: mpfr_rnd_t)
     -> ::libc::c_int;
    pub fn mpfr_j1(arg1: mpfr_ptr, arg2: mpfr_srcptr, arg3: mpfr_rnd_t)
     -> ::libc::c_int;
    pub fn mpfr_jn(arg1: mpfr_ptr, arg2: ::libc::c_long, arg3: mpfr_srcptr,
                   arg4: mpfr_rnd_t) -> ::libc::c_int;
    pub fn mpfr_y0(arg1: mpfr_ptr, arg2: mpfr_srcptr, arg3: mpfr_rnd_t)
     -> ::libc::c_int;
    pub fn mpfr_y1(arg1: mpfr_ptr, arg2: mpfr_srcptr, arg3: mpfr_rnd_t)
     -> ::libc::c_int;
    pub fn mpfr_yn(arg1: mpfr_ptr, arg2: ::libc::c_long, arg3: mpfr_srcptr,
                   arg4: mpfr_rnd_t) -> ::libc::c_int;
    pub fn mpfr_ai(arg1: mpfr_ptr, arg2: mpfr_srcptr, arg3: mpfr_rnd_t)
     -> ::libc::c_int;
    pub fn mpfr_min(arg1: mpfr_ptr, arg2: mpfr_srcptr, arg3: mpfr_srcptr,
                    arg4: mpfr_rnd_t) -> ::libc::c_int;
    pub fn mpfr_max(arg1: mpfr_ptr, arg2: mpfr_srcptr, arg3: mpfr_srcptr,
                    arg4: mpfr_rnd_t) -> ::libc::c_int;
    pub fn mpfr_dim(arg1: mpfr_ptr, arg2: mpfr_srcptr, arg3: mpfr_srcptr,
                    arg4: mpfr_rnd_t) -> ::libc::c_int;
    pub fn mpfr_mul_z(arg1: mpfr_ptr, arg2: mpfr_srcptr, arg3: mpz_srcptr,
                      arg4: mpfr_rnd_t) -> ::libc::c_int;
    pub fn mpfr_div_z(arg1: mpfr_ptr, arg2: mpfr_srcptr, arg3: mpz_srcptr,
                      arg4: mpfr_rnd_t) -> ::libc::c_int;
    pub fn mpfr_add_z(arg1: mpfr_ptr, arg2: mpfr_srcptr, arg3: mpz_srcptr,
                      arg4: mpfr_rnd_t) -> ::libc::c_int;
    pub fn mpfr_sub_z(arg1: mpfr_ptr, arg2: mpfr_srcptr, arg3: mpz_srcptr,
                      arg4: mpfr_rnd_t) -> ::libc::c_int;
    pub fn mpfr_z_sub(arg1: mpfr_ptr, arg2: mpz_srcptr, arg3: mpfr_srcptr,
                      arg4: mpfr_rnd_t) -> ::libc::c_int;
    pub fn mpfr_cmp_z(arg1: mpfr_srcptr, arg2: mpz_srcptr) -> ::libc::c_int;
    pub fn mpfr_mul_q(arg1: mpfr_ptr, arg2: mpfr_srcptr, arg3: mpq_srcptr,
                      arg4: mpfr_rnd_t) -> ::libc::c_int;
    pub fn mpfr_div_q(arg1: mpfr_ptr, arg2: mpfr_srcptr, arg3: mpq_srcptr,
                      arg4: mpfr_rnd_t) -> ::libc::c_int;
    pub fn mpfr_add_q(arg1: mpfr_ptr, arg2: mpfr_srcptr, arg3: mpq_srcptr,
                      arg4: mpfr_rnd_t) -> ::libc::c_int;
    pub fn mpfr_sub_q(arg1: mpfr_ptr, arg2: mpfr_srcptr, arg3: mpq_srcptr,
                      arg4: mpfr_rnd_t) -> ::libc::c_int;
    pub fn mpfr_cmp_q(arg1: mpfr_srcptr, arg2: mpq_srcptr) -> ::libc::c_int;
    pub fn mpfr_cmp_f(arg1: mpfr_srcptr, arg2: mpf_srcptr) -> ::libc::c_int;
    pub fn mpfr_fma(arg1: mpfr_ptr, arg2: mpfr_srcptr, arg3: mpfr_srcptr,
                    arg4: mpfr_srcptr, arg5: mpfr_rnd_t) -> ::libc::c_int;
    pub fn mpfr_fms(arg1: mpfr_ptr, arg2: mpfr_srcptr, arg3: mpfr_srcptr,
                    arg4: mpfr_srcptr, arg5: mpfr_rnd_t) -> ::libc::c_int;
    pub fn mpfr_sum(arg1: mpfr_ptr, arg2: *mut mpfr_ptr,
                    arg3: ::libc::c_ulong, arg4: mpfr_rnd_t) -> ::libc::c_int;
    pub fn mpfr_free_cache();
    pub fn mpfr_subnormalize(arg1: mpfr_ptr, arg2: ::libc::c_int,
                             arg3: mpfr_rnd_t) -> ::libc::c_int;
    pub fn mpfr_strtofr(arg1: mpfr_ptr, arg2: *const ::libc::c_char,
                        arg3: *mut *mut ::libc::c_char, arg4: ::libc::c_int,
                        arg5: mpfr_rnd_t) -> ::libc::c_int;
    pub fn mpfr_custom_get_size(arg1: mpfr_prec_t) -> size_t;
    pub fn mpfr_custom_init(arg1: *mut ::libc::c_void, arg2: mpfr_prec_t);
    pub fn mpfr_custom_get_significand(arg1: mpfr_srcptr)
     -> *mut ::libc::c_void;
    pub fn mpfr_custom_get_exp(arg1: mpfr_srcptr) -> mpfr_exp_t;
    pub fn mpfr_custom_move(arg1: mpfr_ptr, arg2: *mut ::libc::c_void);
    pub fn mpfr_custom_init_set(arg1: mpfr_ptr, arg2: ::libc::c_int,
                                arg3: mpfr_exp_t, arg4: mpfr_prec_t,
                                arg5: *mut ::libc::c_void);
    pub fn mpfr_custom_get_kind(arg1: mpfr_srcptr) -> ::libc::c_int;
    pub fn __gmpfr_set_sj(arg1: mpfr_t, arg2: intmax_t, arg3: mpfr_rnd_t)
     -> ::libc::c_int;
    pub fn __gmpfr_set_sj_2exp(arg1: mpfr_t, arg2: intmax_t, arg3: intmax_t,
                               arg4: mpfr_rnd_t) -> ::libc::c_int;
    pub fn __gmpfr_set_uj(arg1: mpfr_t, arg2: uintmax_t, arg3: mpfr_rnd_t)
     -> ::libc::c_int;
    pub fn __gmpfr_set_uj_2exp(arg1: mpfr_t, arg2: uintmax_t, arg3: intmax_t,
                               arg4: mpfr_rnd_t) -> ::libc::c_int;
    pub fn __gmpfr_mpfr_get_sj(arg1: mpfr_srcptr, arg2: mpfr_rnd_t)
     -> intmax_t;
    pub fn __gmpfr_mpfr_get_uj(arg1: mpfr_srcptr, arg2: mpfr_rnd_t)
     -> uintmax_t;
}
