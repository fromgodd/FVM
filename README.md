# FVM
Fromgodd's Virtual Machine (bytecode)

<img src="https://github.com/fromgodd/FVM/assets/97128346/9b0d751b-fafc-4665-bf46-97829c5258bf_small.png" width="220" height="220">

## Usage
To use the FVM, simply run the following command:

```bash
fvm <filename>
```


Where `<filename>` is the name of the file containing your FVM bytecode.

## Building
To compile the FVM, you can use Cargo. Run the following command:

```bash
cargo build --release
```

## Example FVM Code
This code will just sum up 10 and 12 from stack and print result
```
PUSH 10
PUSH 12
ADD
PRINT
```
