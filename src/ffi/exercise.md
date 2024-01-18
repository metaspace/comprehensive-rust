# Exercise

In this exercise you will implement a safe Rust abstraction for the xnvme
enumerate API. Set up your build environment by cloning and building xnvme and
the exercise source:

```bash
#!/bin/bash
set -e

git clone https://github.com/OpenMPDK/xNVMe xnvme-src
mkdir xnvme-build
mkdir xnvme-install
meson setup -Dwith-spdk=false --prefix $(pwd)/xnvme-install ./xnvme-build ./xnvme-src
meson install -C xnvme-build

git clone https://github.com/metaspace/xnvme-example
cd xnvme-example/exercise

LD_LIBRARY_PATH=$(pwd)/../../xnvme-install/lib PKG_CONFIG_PATH=$(pwd)/../../xnvme-install/lib/pkgconfig cargo run --bin enumerate
```

 You will need the following xnvme C API functions:

```c

struct xnvme_dev;

/**
 * List of devices found on the system usable with xNVMe
 *
 * @struct xnvme_cli_enumeration
 */
struct xnvme_cli_enumeration;

enum xnvme_enumerate_action {
	XNVME_ENUMERATE_DEV_KEEP_OPEN = 0x0, ///< Keep device-handle open after callback returns
	XNVME_ENUMERATE_DEV_CLOSE     = 0x1  ///< Close device-handle when callback returns
};

int
xnvme_cli_enumeration_alloc(struct xnvme_cli_enumeration **list, uint32_t capacity);

void
xnvme_cli_enumeration_free(struct xnvme_cli_enumeration *list);

int
xnvme_cli_enumeration_append(struct xnvme_cli_enumeration *list, const struct xnvme_ident *entry);

/**
 * enumerate devices
 *
 * @param sys_uri URI of the system to enumerate, when NULL, localhost/PCIe
 * @param opts Options for instrumenting the runtime during enumeration
 * @param cb_func Callback function to invoke for each yielded device
 * @param cb_args Arguments passed to the callback function
 *
 * @return On success, 0 is returned. On error, negative `errno` is returned.
 */
int
xnvme_enumerate(const char *sys_uri, struct xnvme_opts *opts, xnvme_enumerate_cb cb_func,
		void *cb_args);


/**
 * Returns the representation of device identifier once decoded from
 * text-representation for the given `dev`
 *
 * @param dev Device handle obtained with xnvme_dev_open() / xnvme_dev_openf()
 *
 * @return On success, device identifier is returned
 */
const struct xnvme_ident *
xnvme_dev_get_ident(const struct xnvme_dev *dev);

/**
 * Destroy the given device handle (::xnvme_dev)
 *
 * @param dev Device handle obtained with xnvme_dev_open()
 */
void
xnvme_dev_close(struct xnvme_dev *dev);
```

The exercise crate in `xnvme-example/exercise` includes auto generated bindings
for these symbols in the `xnvme-sys` namespace.

An unsafe implementation of the enumerate action is available in
`xnvme-example/exercise/src/bin/enumerate.rs`. Implement a safe version in
`xnvme-example/exercise/src/bin/enumerate-rs.rs`

A solution is provided in `xnvme-example/solution`. Try to complete the exercise
on you own, before looking at the solution 😏
