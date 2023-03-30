# bfint

This is a BrainFuck interpreter implemented in Rust. BrainFuck is an esoteric programming language known for its minimalism and extreme simplicity.

## Usage

The bfint-cli run command supports two options:

- `[FILE]`: Specifies the file containing the BrainFuck code to run.
- `-c --code <CODE>`: Positional an argument with BrainFuck code.

Running BrainFuck code with positional argument:
```bash
bfint-cli -c "--<-<<+[+[<+>--->->->-<<<]>]<<--.<++++++.<<-..<<.<+.>>.>>.<<<.+++.>>.>>-.<<<+."
```

Running BrainFuck code from a file:
```bash
bfint-cli ./examples/hello_world.bf
```

## Roadmap
- [x] Write interpreter core
- [x] Write tests for interpreter
- [ ] Add examples
- [x] Add ci/cd 
- [x] Add normal README
- [x] Add normal cli to project

## Contributing
If you'd like to contribute to this project, feel free to fork the repository and submit a pull request.

Please make sure to write tests for any new functionality you add, and ensure that all existing tests continue to pass.

## License
This project is licensed under the MIT License - see the [LICENSE](./LICENSE) file for details.

