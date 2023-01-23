# Camera Interface

This is an interface for a service that provides images from a camera.

![Alt text](images/mechanism.svg)

An actor may either request single images via `Capture` or switch on/off a (potentially unlimited) stream of images via `StreamOn` and `StreamOff`. The actor triggers the __*Camera Provider*__ via a __*Camera Interface*__.

In case the actor requests a stream of images, the __*Camera Provider*__ serves it via a separate interface named __*Image Server*__.

---

 **NOTE:** Currently, this interface does not pass `wash validate`:

```bash
[error] The model has an unresolved reference to shape 'org.wasmcloud.model#Unit'
	Reported by NoUnresolvedReferences.
```

---