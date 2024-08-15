# propresenter-rs

Rust interface for loading ProPresenter files using Google's Protocol Buffers (protobuf).

Utilizes third-party `.proto` files, written by [greyshirtguy](https://github.com/greyshirtguy) on GitHub ([source repo](https://github.com/greyshirtguy/ProPresenter7-Proto)).
Please do not bug greyshirtguy with issues on this package, as he only writes the protobuf files that are used in generated Rust code. As well, do not contact Renewed Vision with issues related to this as they do not support this processing of files.

Taken from greyshirtguy's protobuf README:

> __They are NOT created, endorsed or supported by the makers of ProPresenter: Renewed Vision.__
>
> In addition to the "usual disclaimers" that should obviously apply, please note the following:
> * _If you don't understand what you are doing with these files or you don't have a proven backup and recovery process, you may end up destroying your ProPresenter documents and configuration with no way to recover what you have lost._
> * __⚠️ Do NOT contact Renewed Vision for support!__
> * _Who knows what else could go wrong if you don't understand how to correctly use these files - You might destroy the computer itself, and all other computers on your network (maybe even the Pastor's Macbook). It's possible you might even be responsible for the destruction of millions of other computers around the world and cause the building you are in to burn down. All kidding aside - be careful when changing ProPresenter files - mistakes can crash/break the application_
> * __⚠️ Do NOT contact Renewed Vision for support!__

## Todo

- Potentially switch to [prost](https://crates.io/crates/prost), since it's much better maintained by Tokio compared to [protobuf-codegen](https://crates.io/crates/protobuf-codegen).
- Add examples of loading a presentation from disk to a native Rust struct