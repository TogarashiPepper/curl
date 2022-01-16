# curl
Curl rewritten in rust

To add flags first add the name and type of the option into the args struct:
```diff
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap()]
    URL: String,
    
+    #[clap(short, long)]
+    port: Option<u16>
}
```
then if you flag needs to modify the request add the function to the trait and the trait impl in lib.rs


for info on the types for the option check out [this page on the clap.rs repo](https://github.com/clap-rs/clap/blob/63a36673e1df54baded338b9830aff78e60148ae/examples/derive_ref/README.md#arg-types)
