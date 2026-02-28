cd to rust_sbe_aeron folder which is the base directory of workspace.
Download sbe-all*.jar. This is needed to generate Rust code from xml schema file.
> curl -O https://repo1.maven.org/maven2/uk/co/real-logic/sbe-all/1.37.0/sbe-all-1.37.0.jar

Generate Rust code.
Code will be generated in crate with same name as package below:
```
<?xml version="1.0" encoding="UTF-8"?>
<sbe:messageSchema
	xmlns:sbe="http://fixprotocol.io/2016/sbe"
	package="sales_generated" semanticVersion="1.0.0"
	description="Order message schema demonstrating fixed fields, repeating groups, and variable-length data."
	id="100" version="1">
```
sbe.output.dir is '.'. So crate generated will be in current folder by name 'sales_generated'.
Command to generate crate:
> java -Dsbe.target.language=rust -Dsbe.output.dir=. -jar sbe-all-*.jar ./messages.xml 

**Actually the above is already done, so you can just run test case as below**

There is a single test case in my_sales_app crate under tests/demo_tests.rs.
From CLI run:
> cargo test --package my_sales_app --test demo_test --  --nocapture 
Sample output of test case:
```
     Running tests/demo_test.rs (target/debug/deps/demo_test-920628a1e06276f3)

running 1 test
[25, 0, 1, 0, 100, 0, 1, 0, 234, 0, 0, 0, 0, 0, 0, 0, 135, 0, 0, 0, 0, 0, 0, 0, 246, 0, 0, 0, 0, 0, 0, 0, 0, 14, 0, 2, 222, 0, 0, 0, 2, 0, 234, 0, 0, 0, 0, 0, 0, 0, 111, 0, 0, 0, 3, 0, 123, 0, 0, 0, 0, 0, 0, 0, 13, 0, 100, 117, 99, 107, 32, 105, 115, 32, 97, 110, 103, 114, 121]
template_id = 1, block_length = 25, version = 1
order_id = 234, client_id = 135, timestamp = 246 order_type = New
1. product_id = 222, qty = 2, price = 234 x 10^-2
2. product_id = 111, qty = 3, price = 123 x 10^-2
customer_note = "duck is angry"
test simple ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```