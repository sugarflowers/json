# json crate
## about
項目不定のjsonを扱うためのcrate

## uses

```rust
let mut my_data = JsonData::open("example.json");

let name = my_data.get("name").as_str();
my_data.set("age", 23);

my_data.save("modify.json");
```
