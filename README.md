# Electroplate

Configurable `derive` macros to create encapsulation methods for structs (and maybe enums)

***This project is a significant WIP. APIs and Usage is not finalized until the 1.0.0 release which is still TBD given***

This is partially inspired by [Project Lombok](https://projectlombok.org/) which does a similar thing with Java objects.

## Do you need this library?

It *may* be useful for writing libraries with public APIs but, a similar effect is already provided with how Rust does data access. Given that these are likely optimized away (or can be further through inlining) there is _likely_ no performance impact and using this library is more of a stylistic choice.

This can be used to help teams who are used to the idea of Getter/Setter methods (i.e. POJOs in Java or Properties in C#) write Rust code.

As a note, encapsulation of data in structs (the analogue for objects in other languages) is fairly well covered by member visibility and default immutability.

## Planned Features

The following is currently planned as the scope of Electroplate. This scope may change as development pursues over time:

- [x] Auto-Generation of Struct Member Getters
- [ ] Auto-Generation of Struct Member Getters by Reference
- [ ] Marking Struct Members that are truly private
- [ ] Marking Struct Members that should have an "immutable setter" (Will require `Clone` trait)
- [ ] Marking Struct Members that should have a mutable setter
- [ ]
- [ ] Support for Generic Structs
- [ ] Creating Getters for members of a "child struct" (i.e. `)
- [ ] Custom names for getters/setters
- [ ] Generation of Rustdoc comments*

>\* Rustdoc comments might not be able to include examples. This will be determined when this feature is worked on/designed.

The following is marked as a "maybe", but might be removed for reasons such as feasibility.
- [ ] Support for enums+variants
