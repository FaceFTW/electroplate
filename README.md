# Electroplate

Derive macros to create encapsulation methods for structs (and maybe enums)

This project is a WIP, it might not even materialize into anything

This is partially inspired by [Project Lombok](https://projectlombok.org/) which does a similar thing with Java objects.

## Do you need this library?

Not really (I admit it). Encapsulation of data in structs (the analogue for objects in other languages) is fairly well covered by member visibility and default immutability.

It *may* be useful for writing libraries with public APIs but again, a similar effect is already provided with the framework designed by Rust.

This can be used to help teams who are used to the idea of Getter/Setter methods (i.e. POJOs in Java or Properties in C#) write Rust code.

## Planned Features

The following is currently planned as the scope of Electroplate. This scope may change as development pursues over time:

- Auto-Generation of Struct Member Getters
- Marking Struct Members that are truly private
- Marking Struct Members that should have an "immutable setter" (Will require `Clone` trait)
- Marking Struct Members that should have a mutable setter
- Support for Generic Structs
- Creating Getters for members of a "child struct" (i.e. `)
- Custom names for getters/setters
-


The following is marked as a "maybe", but might be removed for reasons such as feasibility.
- Support for enums+variants
