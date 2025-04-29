# python-crc64fast

Hashlib compatible Python bindings for [crc64fast-nvme](https://github.com/awesomized/crc64fast-nvme).  

## Usage

```Python
import crc64fast

hasher = crc64fast.crc64fast()

hasher.update(b"hello ")
hasher.update(b"world!")

print(hasher.hexdigest())
```

## License

`python-crc64fast` is dual-licensed under

* Apache 2.0 license ([LICENSE-Apache](./LICENSE-Apache) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](./LICENSE-MIT) or <https://opensource.org/licenses/MIT>)
