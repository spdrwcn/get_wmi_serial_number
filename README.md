# `get_wmi_serial_number`


Supported platforms: Any version of Windows that supports WMIC.
## Example

```yaml
use get_wmi_serial_number;

fn main() {
    let serial_number = get_wmi_serial_number::get_bios_serial_number().unwrap();
    println!("{}", serial_number);
}
```


## License

`get_wmi_serial_number` is licensed under both MIT and Apache 2.0
