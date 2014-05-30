/* Copyright (c) 2014, Peter Nelson
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without 
 * modification, are permitted provided that the following conditions are met:
 *
 * 1. Redistributions of source code must retain the above copyright notice, 
 *    this list of conditions and the following disclaimer.
 *
 * 2. Redistributions in binary form must reproduce the above copyright notice,
 *    this list of conditions and the following disclaimer in the documentation
 *    and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
 * AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE
 * LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
 * CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
 * SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
 * INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
 * CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
 * ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
 * POSSIBILITY OF SUCH DAMAGE.
 */
#![crate_id = "jit#0.1.2"]
#![comment = "LibJIT Bindings"]
#![crate_type = "dylib"]
#![crate_type = "rlib"]
#![allow(raw_pointer_deriving)]
#![stable]

//! This crate wraps LibJIT

extern crate libc;

use std::ptr;
use std::mem::transmute;
use libc::{FILE, STDOUT_FILENO, c_int, fdopen, c_void, c_uint, c_char, c_float, c_long};
/// A platform's application binary interface
pub enum ABI {
	/// The C application binary interface
	CDECL = 0
}
/// Call flags to a function
pub enum CallFlags {
	/// When the function won't throw a value
	JitCallNothrow = 1,
	/// When the function won't return a value
	JitCallNoReturn = 2,
	/// When the function is tail-recursive
	JitCallTail = 4,
}

#[link(name = "jit")]
extern {
	fn jit_context_create() -> *c_void;
	fn jit_context_destroy(context: *c_void);
	fn jit_context_build_start(context: *c_void);
	fn jit_context_build_end(context: *c_void);
	fn jit_function_create(context: *c_void, signature: *c_void) -> *c_void;
	fn jit_function_compile(function: *c_void);
	fn jit_function_set_optimization_level(function: *c_void, level: c_uint);
	fn jit_function_set_recompilable(function: *c_void);
	fn jit_function_get_context(function: *c_void) -> *c_void;
	fn jit_function_abandon(function: *c_void) -> *c_void;
	fn jit_type_create_signature(abi: c_int, return_type: *c_void, params: **c_void, num_params: c_uint, incref: c_int) -> *c_void;
	fn jit_type_create_struct(fields: **c_void, num_fields: c_uint, incref: c_int) -> *c_void;
	fn jit_type_create_union(fields: **c_void, num_fields: c_uint, incref: c_int) -> *c_void;
	fn jit_type_create_pointer(pointee: *c_void, incref: c_int) -> *c_void;
	fn jit_type_get_size(ty: *c_void) -> c_uint;
	fn jit_type_get_kind(ty: *c_void) -> c_int;
	fn jit_type_free(ty: *c_void) -> c_void;
	fn jit_value_get_param(function: *c_void, param: c_uint) -> *c_void;
	fn jit_value_get_type(val: *c_void) -> *c_void;
	fn jit_insn_return(function: *c_void, value: *c_void);
	fn jit_insn_throw(function: *c_void, value: *c_void);
	fn jit_insn_default_return(function: *c_void);
	fn jit_function_apply(function: *c_void, args: **c_void, return_area: *mut c_void);
	fn jit_insn_add(function: *c_void, v1: *c_void, v2: *c_void) -> *c_void;
	fn jit_insn_mul(function: *c_void, v1: *c_void, v2: *c_void) -> *c_void;
	fn jit_insn_sub(function: *c_void, v1: *c_void, v2: *c_void) -> *c_void;
	fn jit_insn_div(function: *c_void, v1: *c_void, v2: *c_void) -> *c_void;
	fn jit_insn_and(function: *c_void, v1: *c_void, v2: *c_void) -> *c_void;
	fn jit_insn_or(function: *c_void, v1: *c_void, v2: *c_void) -> *c_void;
	fn jit_insn_xor(function: *c_void, v1: *c_void, v2: *c_void) -> *c_void;
	fn jit_insn_not(function: *c_void, value: *c_void) -> *c_void;
	fn jit_insn_neg(function: *c_void, value: *c_void) -> *c_void;
	fn jit_insn_load(function: *c_void, value: *c_void) -> *c_void;
	fn jit_value_create(function: *c_void, value_type: *c_void) -> *c_void;
	fn jit_insn_label(function: *c_void, label: *mut c_void);
	fn jit_insn_branch(function: *c_void, label: *mut c_void);
	fn jit_insn_le(function: *c_void, v1: *c_void, v2: *c_void) -> *c_void;
	fn jit_insn_ge(function: *c_void, v1: *c_void, v2: *c_void) -> *c_void;
	fn jit_insn_lt(function: *c_void, v1: *c_void, v2: *c_void) -> *c_void;
	fn jit_insn_gt(function: *c_void, v1: *c_void, v2: *c_void) -> *c_void;
	fn jit_insn_eq(function: *c_void, v1: *c_void, v2: *c_void) -> *c_void;
	fn jit_insn_ne(function: *c_void, v1: *c_void, v2: *c_void) -> *c_void;
	fn jit_insn_branch_if(function: *c_void, value: *c_void, label: *mut c_void);
	fn jit_insn_branch_if_not(function: *c_void, value: *c_void, label: *mut c_void);
	fn jit_insn_jump_table(function: *c_void, value: *c_void, labels: *c_void, num_labels: c_uint);
	fn jit_insn_store(function: *c_void, dest: *c_void, src: *c_void);
	fn jit_insn_store_relative(function: *c_void, dest: *c_void, offset: c_int, src: *c_void);
	fn jit_insn_call_native(function: *c_void, name: *c_char, native_func: *u8, signature: *c_void, args: **c_void, num_args: c_uint, flags: c_int) -> *c_void;
	fn jit_insn_call_indirect(function: *c_void, value: *c_void, signature: *c_void, args: **c_void, num_args: c_uint, flags: c_int) -> *c_void;
	fn jit_insn_alloca(function: *c_void, size: *c_void) -> *c_void;
	fn jit_insn_uses_catcher(function: *c_void) -> c_void;
	fn jit_insn_acos(function: *c_void, value: *c_void) -> *c_void;
	fn jit_insn_asin(function: *c_void, value: *c_void) -> *c_void;
	fn jit_insn_atan(function: *c_void, value: *c_void) -> *c_void;
	fn jit_insn_atan2(function: *c_void, value1: *c_void, value2: *c_void) -> *c_void;
	fn jit_insn_ceil(function: *c_void, value: *c_void) -> *c_void;
	fn jit_insn_cos(function: *c_void, value: *c_void) -> *c_void;
	fn jit_insn_cosh(function: *c_void, value: *c_void) -> *c_void;
	fn jit_insn_exp(function: *c_void, value: *c_void) -> *c_void;
	fn jit_insn_floor(function: *c_void, value: *c_void) -> *c_void;
	fn jit_insn_log(function: *c_void, value: *c_void) -> *c_void;
	fn jit_insn_log10(function: *c_void, value: *c_void) -> *c_void;
	fn jit_insn_pow(function: *c_void, value1: *c_void, value2: *c_void) -> *c_void;
	fn jit_insn_rint(function: *c_void, value: *c_void) -> *c_void;
	fn jit_insn_round(function: *c_void, value: *c_void) -> *c_void;
	fn jit_insn_sin(function: *c_void, value: *c_void) -> *c_void;
	fn jit_insn_sinh(function: *c_void, value: *c_void) -> *c_void;
	fn jit_insn_sqrt(function: *c_void, value: *c_void) -> *c_void;
	fn jit_insn_tan(function: *c_void, value: *c_void) -> *c_void;
	fn jit_insn_tanh(function: *c_void, value: *c_void) -> *c_void;
	fn jit_insn_trunc(function: *c_void, value: *c_void) -> *c_void;
	fn jit_insn_is_nan(function: *c_void, value: *c_void) -> *c_void;
	fn jit_insn_is_finite(function: *c_void, value: *c_void) -> *c_void;
	fn jit_insn_is_inf(function: *c_void, value: *c_void) -> *c_void;
	fn jit_insn_abs(function: *c_void, value: *c_void) -> *c_void;
	fn jit_insn_min(function: *c_void, value1: *c_void, value2: *c_void) -> *c_void;
	fn jit_insn_max(function: *c_void, value1: *c_void, value2: *c_void) -> *c_void;
	fn jit_insn_sign(function: *c_void, value: *c_void) -> *c_void;
	fn jit_dump_function (stream: *FILE, funcion: *c_void, name: *c_char);
	fn jit_value_create_float32_constant(function: *c_void, value_type: *c_void, value: c_float) -> *c_void;
	fn jit_value_create_float64_constant(function: *c_void, value_type: *c_void, value: f64) -> *c_void;
	fn jit_value_create_nint_constant(function: *c_void, value_type: *c_void, value: c_int) -> *c_void;
	fn jit_value_create_long_constant(function: *c_void, value_type: *c_void, value: c_long) -> *c_void;
	fn jit_value_create_constant(function: *c_void, constant: *c_void) -> *c_void;
	fn jit_function_to_closure(function: *c_void) -> *c_void;

	static jit_type_void: *c_void;
	static jit_type_sbyte: *c_void;
	static jit_type_ubyte: *c_void;
	static jit_type_short: *c_void;
	static jit_type_ushort: *c_void;
	static jit_type_int: *c_void;
	static jit_type_uint: *c_void;
	static jit_type_nint: *c_void;
	static jit_type_nuint: *c_void;
	static jit_type_long: *c_void;
	static jit_type_ulong: *c_void;
	static jit_type_float32: *c_void;
	static jit_type_float64: *c_void;
	static jit_type_nfloat: *c_void;
	static jit_type_void_ptr: *c_void;
	static jit_type_sys_bool: *c_void;
	static jit_type_sys_char: *c_void;
}
/// Holds all of the functions you have built and compiled. There can be multuple, but normally there is only one.
pub struct Context {
	_context: *c_void
}

impl Context {
	/// Create a new JIT Context
	pub fn new() -> Box<Context> {
		unsafe {
			let context = jit_context_create();
			box Context { _context: context }
		}
	}
	/// Lock down the context to prevent multiple threads from using the builder at a time
	pub fn build_start(&self) {
		unsafe {
			jit_context_build_start(self._context);
		}
	}
	/// Unlock the context from this thread
	pub fn build_end(&self) {
		unsafe {
			jit_context_build_end(self._context);
		}
	}
	/// Create a function in the context with the type signature given
	pub fn create_function(&self, signature: &Type) -> Box<Function> {
		unsafe {
			let function = jit_function_create(self._context, signature._type);
			box Function { _context: self, _function: function }
		}
	}
}

impl Drop for Context {
	fn drop(&mut self) {
		unsafe {
			jit_context_destroy(self._context);
		}
	}
}
/// The types that a value can be
bitflags!(
	flags TypeKind: i32 {
		static Invalid		= -1,
		static Void			= 0,
		static SByte		= 1,
		static UByte		= 2,
		static Short		= 3,
		static UShort		= 4,
		static Int			= 5,
		static UInt 		= 6,
		static NInt 		= 7,
		static NUInt 		= 8,
		static Long 		= 9,
		static ULong 		= 10,
		static Float32 		= 11,
		static Float64 		= 12,
		static NFloat 		= 13,
		static MaxPrimitive = 13,
		static Struct 		= 14,
		static Union 		= 15,
		static Signature 	= 16,
		static Pointer 		= 17,
		static FirstTagged	= 32
	}
)
/// A Type of a value to JIT
pub struct Type {
	_type: *c_void
}
impl Drop for Type {
	fn drop(&mut self) {
		unsafe {
			jit_type_free(self._type);
		}
	}
}
impl Type {
	/// Create a function signature, with the given ABI, return type and parameters
	pub fn create_signature(abi: ABI, return_type: &Type, params: &[&Type]) -> Box<Type> {
		unsafe {
			let mut ps: Vec<*c_void> = vec!();

			for param in params.iter() {
				ps.push(param._type);
			}

			let params = if ps.len() > 0 { ps.as_ptr() } else { 0 as **c_void };

			let signature = jit_type_create_signature(abi as c_int, return_type._type, params, ps.len() as c_uint, 1);
			box Type { _type: signature }
		}
	}

	fn create_complex(fields: &[&Type], union: bool) -> Box<Type> {
		unsafe {
			let mut fs: Vec<*c_void> = vec!();

			for field in fields.iter() {
				fs.push(field._type);
			}

			let fields = if fs.len() > 0 { fs.as_ptr() } else { 0 as **c_void };
			let f = if union { jit_type_create_union } else { jit_type_create_struct };
			let ty = f(fields, fs.len() as c_uint, 1);
			box Type { _type: ty }
		}
	}
	/// Create a struct type with the given field types
	pub fn create_struct(fields: &[&Type]) -> Box<Type> {
		Type::create_complex(fields, false)
	}
	/// Create a union type with the given field types
	pub fn create_union(fields: &[&Type]) -> Box<Type> {
		let inner = Type::create_complex(fields, true);
		Type::create_struct(&[&*Types::get_int(), &*inner])
	}
	/// Create a pointer type with the given pointee type
	pub fn create_pointer(pointee: &Type) -> Box<Type> {
		unsafe {
			let ptr = jit_type_create_pointer(pointee._type, 1);
			box Type { _type: ptr }
		}
	}
	/// Work out the size of this type
	pub fn get_size(&self) -> c_uint {
		unsafe {
			jit_type_get_size(self._type)
		}
	}
	/// Get the kind of this type
	pub fn get_kind(&self) -> TypeKind {
		unsafe {
			TypeKind::from_bits(jit_type_get_kind(self._type)).unwrap()
		}
	}
}
#[deriving(Clone)]
/// A Function to JIT
pub struct Function {
	_context: *Context,
	_function: *c_void
}
impl Drop for Function {
	fn drop(&mut self) {
		unsafe {
			jit_function_abandon(self._function);
		}
	}
}
impl Function {
	fn insn_binop(&self, v1: &Value, v2: &Value, f: unsafe extern "C" fn(function: *c_void, v1: *c_void, v2: *c_void) -> *c_void) -> Box<Value> {
		unsafe {
			let value = f(self._function, v1._value, v2._value);
			box Value { _value: value }
		}
	}

	fn insn_unop(&self, value: &Value, f: unsafe extern "C" fn(function: *c_void, value: *c_void) -> *c_void) -> Box<Value> {
		unsafe {
			let value = f(self._function, value._value);
			box Value { _value: value }
		}
	}
	/// Get the context this function was made i
	pub fn get_context(&self) -> Box<Context> {
		unsafe {
			let context = jit_function_get_context(self._function);
			box Context {_context: context }
		}
	}
	/// Set the optimization level of the function, where the bigger the level, the more effort should be spent optimising
	pub fn set_optimization_level(&self, level: c_uint) {
		unsafe {
			jit_function_set_optimization_level(self._function, level);
		}
	}
	/// Make this funcition a candidate for recompilation
	pub fn set_recompilable(&self) {
		unsafe {
			jit_function_set_recompilable(self._function);
		}
	}
	/// Dump the function onto stdout
	pub fn dump(&self, name: &str) {
		unsafe {
			let c_str = name.to_c_str().unwrap();
			let mode = "w";
			let stdout = fdopen(STDOUT_FILENO, transmute(&mode[0]));
			jit_dump_function(stdout, self._function, c_str);
		}
	}
	/// Compile the function
	pub fn compile(&self) {
		unsafe {
			jit_function_compile(self._function);
		}
	}
	/// Get a parameter of the function as a JIT Value
	pub fn get_param(&self, param: uint) -> Value {
		unsafe {
			let value = jit_value_get_param(self._function, param as c_uint);
			Value { _value: value }
		}
	}
	/// Notify libjit that this function has a catch block in it so it can prepare
	pub fn insn_uses_catcher(&self) {
		unsafe {
			jit_insn_uses_catcher(self._function);
		}
	}
	/// Throw an exception from the function with the value given
	pub fn insn_throw(&self, retval: &Value) {
		unsafe {
			jit_insn_throw(self._function, retval._value);
		}
	}
	/// Return from the function with the value given
	pub fn insn_return(&self, retval: &Value) {
		unsafe {
			jit_insn_return(self._function, retval._value);
		}
	}
	/// Return from the function
	pub fn insn_default_return(&self) {
		unsafe {
			jit_insn_default_return(self._function);
		}
	}
	/// Make an instruction that multiplies the values
	pub fn insn_mul(&self, v1: &Value, v2: &Value) -> Box<Value> {
		self.insn_binop(v1, v2, jit_insn_mul)
	}
	/// Make an instruction that adds the values
	pub fn insn_add(&self, v1: &Value, v2: &Value) -> Box<Value> {
		self.insn_binop(v1, v2, jit_insn_add)
	}
	/// Make an instruction that subtracts the second value from the first
	pub fn insn_sub(&self, v1: &Value, v2: &Value) -> Box<Value> {
		self.insn_binop(v1, v2, jit_insn_sub)
	}
	/// Make an instruction that divides the first number by the second
	pub fn insn_div(&self, v1: &Value, v2: &Value) -> Box<Value> {
		self.insn_binop(v1, v2, jit_insn_div)
	}
	/// Make an instruction that checks if the first value is lower than or equal to the second
	pub fn insn_leq(&self, v1: &Value, v2: &Value) -> Box<Value> {
		self.insn_binop(v1, v2, jit_insn_le)
	}
	/// Make an instruction that checks if the first value is greater than or equal to the second
	pub fn insn_geq(&self, v1: &Value, v2: &Value) -> Box<Value> {
		self.insn_binop(v1, v2, jit_insn_ge)
	}
	/// Make an instruction that checks if the first value is lower than the second
	pub fn insn_lt(&self, v1: &Value, v2: &Value) -> Box<Value> {
		self.insn_binop(v1, v2, jit_insn_lt)
	}
	/// Make an instruction that checks if the first value is greater than the second
	pub fn insn_gt(&self, v1: &Value, v2: &Value) -> Box<Value> {
		self.insn_binop(v1, v2, jit_insn_gt)
	}
	/// Make an instruction that checks if the values are equal
	pub fn insn_eq(&self, v1: &Value, v2: &Value) -> Box<Value> {
		self.insn_binop(v1, v2, jit_insn_eq)
	}
	/// Make an instruction that checks if the values are not equal
	pub fn insn_neq(&self, v1: &Value, v2: &Value) -> Box<Value> {
		self.insn_binop(v1, v2, jit_insn_ne)
	}
	/// Make an instruction that performs a bitwise and on the two values
	pub fn insn_and(&self, v1: &Value, v2: &Value) -> Box<Value> {
		self.insn_binop(v1, v2, jit_insn_and)
	}
	/// Make an instruction that performs a bitwise or on the two values
	pub fn insn_or(&self, v1: &Value, v2: &Value) -> Box<Value> {
		self.insn_binop(v1, v2, jit_insn_or)
	}
	/// Make an instruction that performs a bitwise xor on the two values
	pub fn insn_xor(&self, v1: &Value, v2: &Value) -> Box<Value> {
		self.insn_binop(v1, v2, jit_insn_xor)
	}
	/// Make an instruction that performs a bitwise not on the two values
	pub fn insn_not(&self, value: &Value) -> Box<Value> {
		self.insn_unop(value, jit_insn_not)
	}
	/// Make an instruction that performs a bitwise negate on the value
	pub fn insn_neg(&self, value: &Value) -> Box<Value> {
		self.insn_unop(value, jit_insn_neg)
	}
	/// Make an instruction that duplicates the value given
	pub fn insn_dup(&self, value: &Value) -> Box<Value> {
		unsafe {
			let dup_value = jit_insn_load(self._function, value._value);
			box Value { _value: dup_value }
		}
	}
	/// Make an instruction that stores a value at a destination value
	pub fn insn_store(&self, dest: &Value, src: &Value) {
		unsafe {
			jit_insn_store(self._function, dest._value, src._value);
		}
	}
	/// Make an instruction that stores a value a certain offset away from a destination value
	pub fn insn_store_relative(&self, dest: &Value, offset: c_int, src: &Value) {
		unsafe {
			jit_insn_store_relative(self._function, dest._value, offset, src._value);
		}
	}
	/// Make an instruction that sets a label
	pub fn insn_set_label(&self, label: &mut Label) {
		unsafe {
			// TODO: messy; I suspect transmute is REALLY BAD PRACTICE
			let label_ptr: *mut c_void = transmute(&label._label);
			jit_insn_label(self._function, label_ptr);
		}
	}
	/// Make an instruction that branches to a certain label
	pub fn insn_branch(&self, label: &mut Label) {
		unsafe {
			let label_ptr: *mut c_void = transmute(&label._label);
			jit_insn_branch(self._function, label_ptr);
		}
	}
	/// Make an instruction that branches to a certain label if the value is true
	pub fn insn_branch_if(&self, value: &Value, label: &mut Label) {
		unsafe {
			let label_ptr: *mut c_void = transmute(&label._label);
			jit_insn_branch_if(self._function, value._value, label_ptr);
		}
	}
	/// Make an instruction that branches to a certain label if the value is false
	pub fn insn_branch_if_not(&self, value: &Value, label: &mut Label) {
		unsafe {
			let label_ptr: *mut c_void = transmute(&label._label);
			jit_insn_branch_if_not(self._function, value._value, label_ptr);
		}
	}
	/// Make an instruction that branches to a label in the table
	pub fn insn_jump_table(&self, value: &Value, labels: &[Label]) {
		let mut conv_labels = Vec::with_capacity(labels.len());
		for label in labels.iter() {
			conv_labels.push(label._label);
		}
		unsafe {
			let labels_ptr: *c_void = transmute(conv_labels.as_slice().unsafe_ref(0));
			jit_insn_jump_table(self._function, value._value, labels_ptr, labels.len() as u32);
		}
	}
	/// Make an instruction that calls a function that has the signature given with some arguments
	pub fn insn_call_indirect(&self, func:&Function, signature: &Type, args: &[&Value]) -> Box<Value> {
		unsafe {
			let mut as_: Vec<*c_void> = vec!();

			for arg in args.iter() {
				as_.push(arg._value);
			}

			let args = if as_.len() > 0 { as_.as_ptr() } else { 0 as **c_void };
			box Value {
				_value: jit_insn_call_indirect(self._function, func._function, signature._type, args, as_.len() as c_uint, JitCallNothrow as c_int)
			}
		}
	}
	/// Make an instruction that calls a native function that has the signature given with some arguments
	fn insn_call_native(&self, name: &'static str, native_func: *u8,
						signature: &Type, args: &[&Value]) -> Box<Value> {
		unsafe {
			let mut as_: Vec<*c_void> = vec!();

			for arg in args.iter() {
				as_.push(arg._value);
			}

			let args = if as_.len() > 0 { as_.as_ptr() } else { 0 as **c_void };
			name.with_c_str(|name| {
				box Value {
					_value: jit_insn_call_native(self._function, name, native_func,
												 signature._type, args, as_.len() as c_uint,
												 JitCallNothrow as c_int)
				}
			})
		}
	}
	/// Make an instruction that calls a Rust function that has the signature given with no arguments and expects a return value
	pub fn insn_call_native0<R>(&self, name: &'static str,
								native_func: fn() -> R,
								signature: &Type, args: &[&Value]) -> Box<Value> {
		self.insn_call_native(name, unsafe { transmute(native_func) }, signature, args)
	}
	/// Make an instruction that calls a Rust function that has the signature given with a single argument and expects a return value
	pub fn insn_call_native1<A,R>(&self, name: &'static str,
								  native_func: fn(A) -> R,
								  signature: &Type, args: &[&Value]) -> Box<Value> {
		self.insn_call_native(name, unsafe { transmute(native_func) }, signature, args)
	}
	/// Make an instruction that calls a Rust function that has the signature given with two arguments and expects a return value
	pub fn insn_call_native2<A,B,R>(&self, name: &'static str,
								  native_func: fn(A, B) -> R,
								  signature: &Type, args: &[&Value]) -> Box<Value> {
		self.insn_call_native(name, unsafe { transmute(native_func) }, signature, args)
	}
	/// Make an instruction that calls a Rust function that has the signature given with three arguments and expects a return value
	pub fn insn_call_native3<A,B,C,R>(&self, name: &'static str,
								  native_func: fn(A, B, C) -> R,
								  signature: &Type, args: &[&Value]) -> Box<Value> {
		self.insn_call_native(name, unsafe { transmute(native_func) }, signature, args)
	}
	/// Make an instruction that calls a Rust function that has the signature given with four arguments and expects a return value
	pub fn insn_call_native4<A,B,C,D,R>(&self, name: &'static str,
								  native_func: fn(A, B, C, D) -> R,
								  signature: &Type, args: &[&Value]) -> Box<Value> {
		self.insn_call_native(name, unsafe { transmute(native_func) }, signature, args)
	}
	/// Make an instruction that allocates some space
	pub fn insn_alloca(&self, size: &Value) -> Box<Value> {
		unsafe {
			box Value { _value: jit_insn_alloca(self._function, size._value) }
		}
	}
	/// Apply a function to some arguments and set the retval to the return value
	pub fn apply<T>(&self, args: &[*c_void], retval: &mut T) {
		unsafe {
			let pargs = args;
			jit_function_apply(self._function, pargs.as_ptr(), transmute(retval));
		}
	}
	/// Execute a function and with some arguments
	pub fn execute(&self, args: &[*c_void]) {
		unsafe {
			let pargs = args;
			jit_function_apply(self._function, pargs.as_ptr(), ptr::mut_null());
		}
	}
	/// Turn this function into a closure
	pub fn closure<T>(&self) -> T {
		unsafe {
			transmute(jit_function_to_closure(self._function))
		}
	}
	/// Create a new value with the given type
	pub fn create_value(&self, value_type: &Type) -> Box<Value> {
		unsafe {
			let value = jit_value_create(self._function, value_type._type);
			box Value { _value: value }
		}
	}
	/// Make an instruction that gets the inverse cosine of the number given
	pub fn insn_acos(&self, v: &Value) -> Box<Value> {
		self.insn_unop(v, jit_insn_acos)
	}
	/// Make an instruction that gets the inverse sine of the number given
	pub fn insn_asin(&self, v: &Value) -> Box<Value> {
		self.insn_unop(v, jit_insn_asin)
	}
	/// Make an instruction that gets the inverse tangent of the number given
	pub fn insn_atan(&self, v: &Value) -> Box<Value> {
		self.insn_unop(v, jit_insn_atan)
	}
	/// Make an instruction that gets the inverse tangent of the numbers given
	pub fn insn_atan2(&self, v1: &Value, v2: &Value) -> Box<Value> {
		self.insn_binop(v1, v2, jit_insn_atan2)
	}
	/// Make an instruction that finds the nearest integer above a number
	pub fn insn_ceil(&self, v: &Value) -> Box<Value> {
		self.insn_unop(v, jit_insn_ceil)
	}
	/// Make an instruction that gets the consine of the number given
	pub fn insn_cos(&self, v: &Value) -> Box<Value> {
		self.insn_unop(v, jit_insn_cos)
	}
	/// Make an instruction that gets the hyperbolic consine of the number given
	pub fn insn_cosh(&self, v: &Value) -> Box<Value> {
		self.insn_unop(v, jit_insn_cosh)
	}
	/// Make an instruction that gets the natural logarithm rased to the power of the number
	pub fn insn_exp(&self, v: &Value) -> Box<Value> {
		self.insn_unop(v, jit_insn_exp)
	}
	/// Make an instruction that finds the nearest integer below a number
	pub fn insn_floor(&self, v: &Value) -> Box<Value> {
		self.insn_unop(v, jit_insn_floor)
	}
	/// Make an instruction that gets the natural logarithm of the number
	pub fn insn_log(&self, v: &Value) -> Box<Value> {
		self.insn_unop(v, jit_insn_log)
	}
	/// Make an instruction that gets the base 10 logarithm of the number
	pub fn insn_log10(&self, v: &Value) -> Box<Value> {
		self.insn_unop(v, jit_insn_log10)
	}
	/// Make an instruction the gets the result of raising the first value to the power of the second value
	pub fn insn_pow(&self, v1: &Value, v2:&Value) -> Box<Value> {
		self.insn_binop(v1, v2, jit_insn_pow)
	}
	/// Make an instruction the gets the result of rounding the value to the nearest integer
	pub fn insn_rint(&self, v: &Value) -> Box<Value> {
		self.insn_unop(v, jit_insn_rint)
	}
	/// Make an instruction the gets the result of rounding the value to the nearest integer
	pub fn insn_round(&self, v: &Value) -> Box<Value> {
		self.insn_unop(v, jit_insn_round)
	}
	/// Make an instruction the gets the sine of the number
	pub fn insn_sin(&self, v: &Value) -> Box<Value> {
		self.insn_unop(v, jit_insn_sin)
	}
	/// Make an instruction the gets the hyperbolic sine of the number
	pub fn insn_sinh(&self, v: &Value) -> Box<Value> {
		self.insn_unop(v, jit_insn_sinh)
	}
	/// Make an instruction the gets the square root of a number
	pub fn insn_sqrt(&self, v: &Value) -> Box<Value> {
		self.insn_unop(v, jit_insn_sqrt)
	}
	/// Make an instruction the gets the tangent of a number
	pub fn insn_tan(&self, v: &Value) -> Box<Value> {
		self.insn_unop(v, jit_insn_tan)
	}
	/// Make an instruction the gets the hyperbolic tangent of a number
	pub fn insn_tanh(&self, v: &Value) -> Box<Value> {
		self.insn_unop(v, jit_insn_tanh)
	}
	/// Make an instruction that truncates the value
	pub fn insn_trunc(&self, v: &Value) -> Box<Value> {
		self.insn_unop(v, jit_insn_trunc)
	}
	/// Make an instruction that checks if the number is NaN
	pub fn insn_is_nan(&self, v: &Value) -> Box<Value> {
		self.insn_unop(v, jit_insn_is_nan)
	}
	/// Make an instruction that checks if the number is finite
	pub fn insn_is_finite(&self, v: &Value) -> Box<Value> {
		self.insn_unop(v, jit_insn_is_finite)
	}
	/// Make an instruction that checks if the number is  infinite
	pub fn insn_is_inf(&self, v: &Value) -> Box<Value> {
		self.insn_unop(v, jit_insn_is_inf)
	}
	/// Make an instruction that gets the absolute value of a number
	pub fn insn_abs(&self, v: &Value) -> Box<Value> {
		self.insn_unop(v, jit_insn_abs)
	}
	/// Make an instruction that gets the smallest of two numbers
	pub fn insn_min(&self, v1: &Value, v2: &Value) -> Box<Value> {
		self.insn_binop(v1, v2, jit_insn_min)
	}
	/// Make an instruction that gets the biggest of two numbers
	pub fn insn_max(&self, v1: &Value, v2: &Value) -> Box<Value> {
		self.insn_binop(v1, v2, jit_insn_max)
	}
	/// Make an instruction that gets the sign of a number
	pub fn insn_sign(&self, v: &Value) -> Box<Value> {
		self.insn_unop(v, jit_insn_sign)
	}
	/// Convert it into a value
	pub fn as_value(&self) -> Box<Value> {
		box Value {
			_value: self._function
		}
	}
}
#[deriving(Clone)]
/// A Value that is being JITed
pub struct Value {
	_value: *c_void
}
impl Value {
	/// Get the type of the value
	pub fn get_type(&self) -> Box<Type> {
		unsafe {
			let ty = jit_value_get_type(self._value);
			box Type { _type: ty }
		}
	}
}

#[deriving(Hash, Eq)]
/// A label in the code that can be branched to in instructions
pub struct Label {
	_label: *c_void
}

impl Label {
	fn undefined() -> *c_void {
		!0u32 as *c_void
	}
	/// Create a new label
	pub fn new() -> Box<Label> {
		box Label { _label: Label::undefined() }
	}
}
/// Holds type constructors
pub struct Types;
impl Types {
	/// Void type
	pub fn get_void() -> Box<Type> {
		box Type { _type: jit_type_void }
	}
	/// Integer type
	pub fn get_int() -> Box<Type> {
		box Type { _type: jit_type_int }
	}
	/// Unsigned integer type
	pub fn get_uint() -> Box<Type> {
		box Type { _type: jit_type_uint }
	}
	/// Long integer type
	pub fn get_long() -> Box<Type> {
		box Type { _type: jit_type_long }
	}
	/// Unsigned long integer type
	pub fn get_ulong() -> Box<Type> {
		box Type { _type: jit_type_ulong }
	}
	/// 32-bit floating point type
	pub fn get_float32() -> Box<Type> {
		box Type { _type: jit_type_float32 }
	}
	/// 64-bit floating point type
	pub fn get_float64() -> Box<Type> {
		box Type { _type: jit_type_float64 }
	}
	/// Default floating point type
	pub fn get_float() -> Box<Type> {
		box Type { _type: jit_type_nfloat }
	}
	/// A void pointer, which can represent any kind of pointer
	pub fn get_void_ptr() -> Box<Type> {
		box Type { _type: jit_type_void_ptr }
	}
	/// Character type
	pub fn get_char() -> Box<Type> {
		box Type { _type: jit_type_sys_char }
	}
	/// C String type
	pub fn get_cstring() -> Box<Type> {
		Type::create_pointer(&*Types::get_char())
	}
	/// Boolean type
	pub fn get_bool() -> Box<Type> {
		box Type { _type: jit_type_sys_bool }
	}
	/// Vec<?> type
	pub fn get_vec() -> Box<Type> {
		let uint_t = Types::get_uint();
		let void_ptr_t = Types::get_void_ptr();
		Type::create_struct(&[&*uint_t, &*uint_t, &*void_ptr_t])
	}
}
struct PointerConstant {
	_type: *c_void,
	_ptr: *c_void
}
/// A type that can be compiled into a LibJIT representation
pub trait Compilable {
	/// Get a JIT representation of this value
	fn compile(&self, func:&Function) -> Box<Value>;
}
impl Compilable for () {
	fn compile(&self, func:&Function) -> Box<Value> {
		unsafe {
			box Value {
				_value: jit_value_create_nint_constant(func._function, jit_type_void_ptr, 0)
			}
		}
	}
}
impl Compilable for f64 {
	fn compile(&self, func:&Function) -> Box<Value> {
		unsafe {
			box Value {
				_value: jit_value_create_float64_constant(func._function, jit_type_float64, *self) 
			}
		}
	}
}
impl Compilable for f32 {
	fn compile(&self, func:&Function) -> Box<Value> {
		unsafe {
			box Value {
				_value: jit_value_create_float32_constant(func._function, jit_type_float32, *self) 
			}
		}
	}
}
impl Compilable for int {
	fn compile(&self, func:&Function) -> Box<Value> {
		unsafe {
			box Value {
				_value: jit_value_create_long_constant(func._function, jit_type_nint, *self as i64) 
			}
		}
	}
}
impl Compilable for uint {
	fn compile(&self, func:&Function) -> Box<Value> {
		unsafe {
			box Value {
				_value: jit_value_create_nint_constant(func._function, jit_type_nuint, *self as i32) 
			}
		}
	}
}
impl Compilable for i32 {
	fn compile(&self, func:&Function) -> Box<Value> {
		unsafe {
			box Value {
				_value: jit_value_create_nint_constant(func._function, jit_type_int, *self) 
			}
		}
	}
}
impl Compilable for u32 {
	fn compile(&self, func:&Function) -> Box<Value> {
		unsafe {
			box Value {
				_value: jit_value_create_nint_constant(func._function, jit_type_uint, *self as i32) 
			}
		}
	}
}
impl Compilable for i16 {
	fn compile(&self, func:&Function) -> Box<Value> {
		unsafe {
			box Value {
				_value: jit_value_create_nint_constant(func._function, jit_type_short, *self as i32) 
			}
		}
	}
}
impl Compilable for u16 {
	fn compile(&self, func:&Function) -> Box<Value> {
		unsafe {
			box Value {
				_value: jit_value_create_nint_constant(func._function, jit_type_ushort, *self as i32) 
			}
		}
	}
}
impl Compilable for i8 {
	fn compile(&self, func:&Function) -> Box<Value> {
		unsafe {
			box Value {
				_value: jit_value_create_nint_constant(func._function, jit_type_sbyte, *self as i32) 
			}
		}
	}
}
impl Compilable for u8 {
	fn compile(&self, func:&Function) -> Box<Value> {
		unsafe {
			box Value {
				_value: jit_value_create_nint_constant(func._function, jit_type_ubyte, *self as i32) 
			}
		}
	}
}
impl Compilable for bool {
	fn compile(&self, func:&Function) -> Box<Value> {
		unsafe {
			box Value {
				_value: jit_value_create_nint_constant(func._function, jit_type_sys_bool, *self as i32) 
			}
		}
	}
}
impl Compilable for char {
	fn compile(&self, func:&Function) -> Box<Value> {
		unsafe {
			box Value {
				_value: jit_value_create_nint_constant(func._function, jit_type_ubyte, *self as i32) 
			}
		}
	}
}
impl<'t> Compilable for &'t str {
	fn compile(&self, func:&Function) -> Box<Value> {
		let cstring_t = Types::get_cstring();
		let strlen_i = (self.len() as i32).compile(func);
		let bufptr = func.create_value(cstring_t);
		func.insn_store(bufptr, func.insn_alloca(&*strlen_i));
		for i in range(0, self.len()) {
			let char_i = self.char_at(i).compile(func);
			func.insn_store_relative(bufptr, i as i32, char_i);
		}
		let null_term = '\0'.compile(func);
		func.insn_store_relative(bufptr, self.len() as i32, null_term);
		bufptr
	}
}
impl Compilable for String {
	fn compile(&self, func:&Function) -> Box<Value> {
		self.as_slice().compile(func)
	}
}
impl<'t, T> Compilable for &'t T {
	fn compile(&self, func:&Function) -> Box<Value> {
		unsafe {
			box Value {
				_value: jit_value_create_constant(func._function, transmute(&PointerConstant {
					_type: jit_type_void_ptr,
					_ptr: transmute(*self)
				}))
			}
		}
	}
}