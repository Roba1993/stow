# stow
Cloud storage abstraction package for Rust

## Implementations

Supported endpoints:
* Local (folders are containers, files are items)
* Google Cloud Storage

Additional endpoints can be added if needed.

## Concepts

The concepts of Stow are modeled around the most popular object storage services, and are made up of three main objects:

* `Location` - a place where many `Container` objects are stored
* `Container` - a named group of `Item` objects
* `Item` - an individual file

```
location1 (e.g. GCS)
├── container1
├───── item1.1
├───── item1.2
├───── item1.3
├── container2
├───── item2.1
├───── item2.2
location2 (e.g. local storage)
├── container1
├───── item1.1
├───── item1.2
├───── item1.3
├── container2
├───── item2.1
├───── item2.2
```

* A location contains many containers
* A container contains many items
* Containers do not contain other containers
* Items must belong to a container
* Item names may be a path

## Thanks

A big thanks to the [original stow implementation in go](https://github.com/graymeta/stow)