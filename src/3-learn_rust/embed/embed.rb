require 'ffi'

module Hello
    extend FFI::Library
    # Load up shared object library.
    ffi_lib 'target/release/libembed.so'
    # connects Rust's process to a Ruby function.
    # First arg is the name of the Rust function,
    # second is the argument list,
    # third is the return type.
    attach_function :process, [], :void
end

# Calls Rust.
Hello.process

puts 'done!'
