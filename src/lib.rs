use valkey_module::alloc::ValkeyAlloc;
use valkey_module::{valkey_module, Context, ValkeyError, ValkeyResult, ValkeyString, ValkeyValue};

fn abyssal_add(_: &Context, args: Vec<ValkeyString>) -> ValkeyResult {
    if args.len() != 3 {
        return Err(ValkeyError::WrongArity);
    }

    let nums = args.into_iter().skip(1).map(|s| s.parse_integer()).collect::<Result<Vec<i64>, ValkeyError>>()?;
    let summation = nums.iter().sum();
    let mut resp = nums;
    resp.push(summation);

    Ok(resp.into())
}

fn abyssal_life(_: &Context, _: Vec<ValkeyString>) -> ValkeyResult {
    let forty_two = ValkeyValue::Integer(42);
    Ok(forty_two)
}


////////////////
////////////////
////////////////

valkey_module! {
    name: "abyssal",
    version: 1,
    allocator: (ValkeyAlloc, ValkeyAlloc),
    data_types: [],
    commands: [
        ["abyssal.add", abyssal_add, "", 0, 0, 0],
        ["abyssal.life", abyssal_life, "", 0, 0, 0],
    ]
}
