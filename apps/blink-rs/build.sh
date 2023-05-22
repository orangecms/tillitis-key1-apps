cargo clean

cargo rustc --release --target=riscv32i-unknown-none-elf -- \
  --emit asm/opt/homebrew/opt/llvm/bin/clang -target  riscv32-unknown-none-elf -march=rv32iczmmul -mabi=ilp32 -mcmodel=medany \
  -static -std=gnu99 -O2 -ffast-math -fno-common -fno-builtin-printf \
  -fno-builtin-putchar -nostdlib -mno-relax -flto -g \
  -Wall -Werror=implicit-function-declaration \
  -I ../../../tkey-libs target/riscv32i-unknown-none-elf/release/deps/app-*.s -o app.elf

/opt/homebrew/opt/llvm/bin/llvm-objcopy --input-target=elf32-littleriscv --output-target=binary app.elf app.bin

