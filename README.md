# `get_wmi_serial_number`


Supported platforms: Any version of Windows that supports WMIC.
## Example

```yaml
use get_wmi_serial_number;

fn main() {
    let serialnumber = get_bios_serial_number();
    println!("{}", serialnumber);
}
```


## License

`get_wmi_serial_number` is licensed under both MIT and Apache 2.0
