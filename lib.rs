use std::collections::HashMap;

pub fn tests() -> HashMap<&'static str, &'static str> {

    let mut result = HashMap::new();
    
    result.insert("address", include_str!("ml-proto/test/address.wast"));
    result.insert("conversions", include_str!("ml-proto/test/conversions.wast"));
    result.insert("endianness", include_str!("ml-proto/test/endianness.wast"));
    result.insert("exports", include_str!("ml-proto/test/exports.wast"));
    result.insert("f32", include_str!("ml-proto/test/f32.wast"));
    result.insert("f32_cmp", include_str!("ml-proto/test/f32_cmp.wast"));
    result.insert("f64", include_str!("ml-proto/test/f64.wast"));
    result.insert("f64_cmp", include_str!("ml-proto/test/f64_cmp.wast"));
    result.insert("fac", include_str!("ml-proto/test/fac.wast"));
    result.insert("float_literals", include_str!("ml-proto/test/float_literals.wast"));
    result.insert("float_memory", include_str!("ml-proto/test/float_memory.wast"));
    result.insert("float_misc", include_str!("ml-proto/test/float_misc.wast"));
    result.insert("forward", include_str!("ml-proto/test/forward.wast"));
    result.insert("func_ptrs", include_str!("ml-proto/test/func_ptrs.wast"));
    result.insert("functions", include_str!("ml-proto/test/functions.wast"));
    result.insert("has_feature", include_str!("ml-proto/test/has_feature.wast"));
    result.insert("i32", include_str!("ml-proto/test/i32.wast"));
    result.insert("i64", include_str!("ml-proto/test/i64.wast"));
    result.insert("imports", include_str!("ml-proto/test/imports.wast"));
    result.insert("int_exprs", include_str!("ml-proto/test/int_exprs.wast"));
    result.insert("int_literals", include_str!("ml-proto/test/int_literals.wast"));
    result.insert("labels", include_str!("ml-proto/test/labels.wast"));
    result.insert("left-to-right", include_str!("ml-proto/test/left-to-right.wast"));
    result.insert("memory", include_str!("ml-proto/test/memory.wast"));
    result.insert("memory_redundancy", include_str!("ml-proto/test/memory_redundancy.wast"));
    result.insert("memory_trap", include_str!("ml-proto/test/memory_trap.wast"));
    result.insert("names", include_str!("ml-proto/test/names.wast"));
    result.insert("nan-propagation", include_str!("ml-proto/test/nan-propagation.wast"));
    result.insert("resizing", include_str!("ml-proto/test/resizing.wast"));
    result.insert("runaway-recursion", include_str!("ml-proto/test/runaway-recursion.wast"));
    result.insert("select", include_str!("ml-proto/test/select.wast"));
    result.insert("store_retval", include_str!("ml-proto/test/store_retval.wast"));
    result.insert("switch", include_str!("ml-proto/test/switch.wast"));
    result.insert("traps", include_str!("ml-proto/test/traps.wast"));
    result.insert("unreachable", include_str!("ml-proto/test/unreachable.wast"));

    result

}
