# Lab 5 Report

## Zhaoyi Wang 1689747

------

## Part 1: Starting Web Assembly

------

> https://github.com/WebAssembly/design/blob/main/Semantics.md
>
> https://www.wasm.com.cn/docs/text-format/

The answer is:

```
(module
 (func $RecursiveCount (param $A i32) (result i32)
			get_local $A
   		i32.const 10
    	i32.gt_s
	if (result i32)
			i32.const 0
	else
			get_local $A 
   		get_local $A 
   		i32.const 1
			i32.add
			call $RecursiveCount
			i32.add
	end)
 (export "RecursiveCount" (func $RecursiveCount)))
```

Code comments:

```
(module
 (func $RecursiveCount (param $A i32) (result i32)
			get local parameter $A
   		set a constant value 10 <i32>
    	compare the $A with 10, condition is "$A is bigger than 10 "
	if the condition is satisfied (result i32)
			return 0 <i32>
	else
			get local parameter $A (1st)
   		get local parameter $A (2nd)
   		set a constant value 1 <i32>
			Add 1 to 2nd $A
			call function RecursiveCount
			Add the result to 1st $A
	end)
 (export "RecursiveCount" (func $RecursiveCount)))
```

------

## Part 2: Hello, World !

------

![image-20211123112030756](/Users/wangzhaoyi/Library/Application Support/typora-user-images/image-20211123112030756.png)

------

## Part 3: Implementing is-prime app

------

In `wasm-is-prime/src/lib.rs`:

```rust
mod utils;

use wasm_bindgen::prelude::*;
use prime_tools::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, wasm-is-prime! ©Zhaoyi");
}

#[wasm_bindgen]
pub fn CheckPrime(s: &JsValue) {
    let mut input: String = s.as_string().unwrap();
    if is_prime(input) {
        alert("Input is Prime");
    } else {
        alert("Input is NOT Prime");
    }
}
pub fn is_prime(s: String) -> bool {
    let input: u32 = s.trim().parse().expect("Cannot parse");
    is_u32_prime(input)
}
```

In `wasm-is-prime/www/index.html`:

```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="utf-8">
    <title>Is-Prime APP!</title>
</head>
<body>
    <noscript>This page contains webassembly and javascript content, please enable javascript in your browser.</noscript>
    <script src="./bootstrap.js"></script>
    <form>
        Enter a number: <input type="text" value="2" id="PrimeNumber"><br>
    </form>
    <button id="CheckNumber">Check Number</button>
</body>
</html>
```

In `wasm-is-prime/www/index.js`:

```js
import * as wasm from "wasm-is-prime";

// wasm.greet();
const textbox1 = document.getElementById("PrimeNumber");
document.getElementById("CheckNumber").addEventListener("click", event => {
    wasm.CheckPrime(textbox1.value);
});
```

In `wasm-is-prime/pkg/wasm-is-prime.d.ts`:

```typescript
export function greet(): void;
export function CheckPrime(s: any): void;  // ADD this line
```

Now, we run `wasm-pack build` under root path `/wasm-is-prime`, and then run `npm run start` under path `wasm-is-prime/www/`.

**The output**

![image-20211123121729476](/Users/wangzhaoyi/Library/Application Support/typora-user-images/image-20211123121729476.png)

![image-20211123121742827](/Users/wangzhaoyi/Library/Application Support/typora-user-images/image-20211123121742827.png)
