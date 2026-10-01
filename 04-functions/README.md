# Rust Tutorial Project — Principles of Programming Languages

> **กลุ่มที่:** `04`<br>
> **Topic No.:** `04`<br>
> **Topic Name:** `Functions`<br>
> **ประเด็นหลักที่ควรครอบคลุม:** `function declaration, parameters, return values, expressions ใน function, scope`<br>

---

## 1. Members

| # | Name | Student ID | GitHub Username | Main Responsibility |
|---|---|---|---|---|
| 1 | นายจักรพรรดิ โพธิพิภัทรกุล | 660710976 | `@660710976` | Concept + Short Code Illustration (สรุปแนวคิดหลัก + โค้ดตัวอย่างสั้น) |
| 2 | นายกฤตเมธ ไทยภักดี | 670710121 | `@670710121` | Detailed Code + Live Demo (โค้ดเชิงลึก + สาธิตสด) |
| 3 | นางสาวกัญญรัชต์ สมหวังพรเจริญ | 670710122 | `@670710122` | Rust vs Other Language + PPL Analysis (เปรียบเทียบภาษา + วิเคราะห์เชิง PPL) |
| 4 | นางสาวกัญญาณัฐ เขมนรากร | 670710123 | `@670710123` | Exercises + Common Mistakes + Challenge (แบบฝึกหัด + ข้อผิดพลาดที่พบบ่อย + คำถามท้าทาย) |

> แก้ไข GitHub Username ของแต่ละคนให้ตรงกับบัญชีจริงก่อนเริ่มทำงาน (ผู้สอนจะใช้คอลัมน์นี้เชิญเป็น collaborator ของ repository)

---

## 2. Learning Objectives

หลังจากศึกษา Topic นี้แล้ว ผู้เรียนสามารถ:

1. `[อธิบายแนวคิดสำคัญได้]`
2. `[เขียนโปรแกรม Rust ที่เกี่ยวข้องได้]`
3. `[วิเคราะห์พฤติกรรม/กฎของภาษาได้]`
4. `[เปรียบเทียบ Rust กับภาษาอื่นได้]`

---

## 3. Introduction

`Function ในภาษา Rust คือ ชุดคำสั่งย่อยที่ประกาศด้วย fn ใช้จัดระเบียบตรรกะและสามารถเรียกใช้งานซ้ำได้`<br>
`ช่วยให้โค้ดเป็นระเบียบ อ่านง่าย ลดความซ้ำซ้อน และปลอดภัยด้วยระบบตรวจสอบชนิดข้อมูล (Type System) ที่เข้มงวด`<br>
`แก้ปัญหาโค้ดซ้ำซ้อน (Duplication), โค้ดซับซ้อนแก้ไขยาก และข้อผิดพลาดจากการส่งชนิดข้อมูลผิดพลาดตั้งแต่ตอนคอมไพล์`

---

## 4. Key Concepts

### 4.1 `[การประกาศฟังก์ชัน (Function Declaration)]`

**คำอธิบาย**

`ใช้คีย์เวิร์ด fn ตามด้วยชื่อฟังก์ชัน` <br>
`ต้องระบุชนิดข้อมูล (Data Type) ของพารามิเตอร์ทุกตัวอย่างชัดเจนเสมอ` <br>
`หากฟังก์ชันมีการคืนค่า (Return value) ต้องใส่เครื่องหมาย -> ตามด้วยชนิดข้อมูลที่คืนค่า`

**ตัวอย่าง**

```rust
fn add_numbers(a: i32, b: i32) -> i32 {
    a + b
}
```

**Explanation**

`fn add_numbers(a: i32, b: i32) -> i32: ประกาศฟังก์ชันชื่อ add_numbers รับพารามิเตอร์ a และ b ชนิด i32 และกำหนดให้คืนค่า (Return) ออกมาเป็นชนิด i32` <br>
`a + b: เป็นการคืนค่าผลบวกของตัวแปร a และ b`

---

### 4.2 `Statement vs Expression (หัวใจสำคัญของ Rust)`

`ภาษา Rust แยกแยะระหว่าง Statement และ Expression อย่างชัดเจน ซึ่งส่งผลต่อการคืนค่าของฟังก์ชัน:` <br><br>

`Statement: คือคำสั่งที่ทำงานบางอย่างแต่ ไม่คืนค่า มักจะลงท้ายด้วยเครื่องหมายเซมิโคลอน ;`<br>
`Expression: คือนิพจน์ที่ ประเมินผลและคืนค่าออกมา (เช่น 5 + 6, การเรียกฟังก์ชัน, หรือแม้แต่บล็อกโค้ด { let x = 3; x + 1 })` <br><br>

`กฎการ Return: บรรทัดสุดท้ายของฟังก์ชันใน Rust หาก ไม่มีเซมิโคลอน (;) ต่อท้าย จะถือว่าเป็น Expression และถูกใช้เป็นค่า Return ของฟังก์ชันนั้นทันที (ไม่ต้องใช้คำว่า return) แต่ถ้าใส่ ; จะกลายเป็น Statement ที่ไม่คืนค่า (ซึ่งถ้าฟังก์ชันต้องการคืนค่า i32 แต่ดันใส่ ; จะเกิด Error ทันที)`

---

### 4.3 `[การคืนค่าหลายค่า (Multiple Return Values)]`

`Rust ไม่มี Syntax พิเศษสำหรับการคืนค่าหลายค่าโดยตรง แต่เราสามารถใช้ Tuple หรือ Struct เพื่อคืนค่ามากกว่าหนึ่งค่าได้อย่างง่ายดาย`

```rust
fn get_user_info() -> (String, u32) {

    ("Alice".to_string(), 30)
}
```

---

### 4.4 `[การจัดการ Ownership และ Borrowing กับ Function]`

`เนื่องจาก Rust ใช้ระบบ Memory Management แบบ Ownership การส่งตัวแปร (เช่น String หรือ Vector) เข้าไปในฟังก์ชัน จะมีผลต่อสิทธิ์การใช้งานตัวแปรนั้น:` <br><br>

`Move: ถ้าส่งตัวแปรเข้าไปตรงๆ (เช่น print_string(s)) สิทธิ์ความเป็นเจ้าของจะย้าย (Move) เข้าไปในฟังก์ชัน ตัวแปรเดิมข้างนอกจะใช้งานต่อไม่ได้` <br>
`Borrowing (References): หากต้องการให้ฟังก์ชันยืมไปใช้งานเฉยๆ โดยที่ข้างนอกยังใช้ต่อได้ ให้ส่งผ่าน Reference (& หรือ &mut) แทน`

```rust
fn print_length(s: &String) {
    println!("Length: {}", s.len());
}

fn main() {
    let my_string = String::from("Hello");
    print_length(&my_string);
    println!("{}", my_string);
}
```

---

### 4.5 `Diverging Functions (ฟังก์ชันที่ไม่เคยคืนค่า)`

`Rust มีฟังก์ชันพิเศษที่เรียกว่า Diverging function ซึ่งใช้เครื่องหมาย ! เป็น Return type หมายความว่าฟังก์ชันนี้ทำงานแล้วไม่มีวันจบปกติ (เช่น ฟังก์ชันที่โปรแกรมจะพังหรือวนลูปไม่รู้จบ)`

```rust
fn panic_error() -> ! {
    panic!("Crash program!");
}
```

---

## 5. Important Syntax / Rules

| Syntax / Rule | Meaning | Example |
|---|---|---|
| `fn name(param: Type) -> ReturnType` | `การประกาศฟังก์ชัน กำหนดชื่อ พารามิเตอร์พร้อม Type และชนิดข้อมูลที่ต้องคืนค่า` | `fn add(a: i32, b: i32) -> i32` |
| `บรรทัดสุดท้าย ไม่มี เซมิโคลอน (;)` | `ใช้เป็น Expression เพื่อคืนค่าอัตโนมัติ (Implicit Return) โดยไม่ต้องใช้คำสั่ง return` | `fn square(x: i32) -> i32 { x * x }` |
| `Return Type เป็นเครื่องหมายตกใจ -> !` | `Diverging Function ฟังก์ชันที่ไม่เคยคืนค่าปกติกลับมา (เช่น Panic หรือ Infinite Loop)` | `fn fail() -> ! { panic!("Error"); }` |
| `&mut T` | `Mutable Reference: การส่ง Reference เข้าฟังก์ชันเพื่อให้ฟังก์ชันสามารถแก้ไขค่าของตัวแปรต้นฉบับได้` | `fn update(s: &mut String) { s.push_str("!"); }` |
| `pub fn / pub(crate) fn` | `Visibility (การมองเห็น): กำหนดสิทธิ์การเข้าถึงฟังก์ชันจากภายนอกโมดูลหรือ crate` | `pub fn calculate() {}` |
| `dyn Trait / impl Trait` | `Generics & Traits: การสร้างฟังก์ชันที่รับหรือคืนค่าได้หลาย Type แบบ Polymorphism` | `fn print_item<T: Display>(item: T) {}` |
| `Closures (\|params\| body)` | `Anonymous Functions: ฟังก์ชันไม่มีชื่อที่สามารถจับตัวแปรจาก Environment รอบข้างได้` | `let add = \|a, b\| a + b;` |

### Important Rules

1. `Type Annotation is Mandatory: พารามิเตอร์ทุกตัวของฟังก์ชันในภาษา Rust ต้องระบุ Data Type เสมอ (คอมไพเลอร์จะไม่ช่วยเดา Type ให้เหมือนกับตัวแปรทั่วไปที่ใช้ let)`
2. `Statement vs Expression: ห้ามใส่เซมิโคลอน (;) ที่บรรทัดสุดท้ายของฟังก์ชันหากต้องการให้บรรทัดนั้นเป็นค่า Return ถ้าใส่จะกลายเป็น Statement และทำให้เกิดคอมไพล์เออร์เออร์เรอร์หากฟังก์ชันนั้นกำหนด Return Type ไว้`
3. `Ownership Transfer by Default: การส่งค่าตัวแปรประเภทที่ไม่ใช่ Primitive (เช่น String หรือ Vector) เข้าไปในฟังก์ชันแบบปกติจะทำให้เกิดการย้ายสิทธิ์ (Move) และไม่สามารถนำตัวแปรนั้นกลับมาใช้ซ้ำข้างนอกได้ เว้นแต่จะใช้ Reference (&) เพื่อยืมค่าแทน`
4. `Method vs Function: ฟังก์ชันที่ถูกผูกไว้กับ Struct หรือ Enum (ประกาศภายในบล็อก impl) ซึ่งจะมีพารามิเตอร์ตัวแรกเป็น self, &self, หรือ &mut self`
5. `Higher-Order Functions: ความสามารถในการรับฟังก์ชันอื่นเป็นพารามิเตอร์ หรือการคืนค่าฟังก์ชันออกจากฟังก์ชัน (มักใช้คู่กับ Closures และ Iterator เช่น .map() หรือ .filter())`

---

## 6. Runnable Code Examples

> **ข้อกำหนด:** Code ทุกตัวต้อง Compile และ Run ได้จริงก่อนนำมาใส่ในเอกสาร

### Example 1 — `Factorial`

**Purpose:** `การสร้าง Function การคำนวนอะไรบ้างอย่างในที่นี้ขอยกเป็นการหาค่า Factorial`

```rust
fn main() {
    let x = factorial(5);
    print!("{}", x)
}

fn factorial(num: i32) -> i32 {
    let mut result = 1;
    if num == 0 {
        return result;
    }
    else {
        for i in 1..=num {
            result *= i
        }
        return result;
    }
}
```

**Expected Output**

```text
120
```

**Explanation**

`factorial(num): ฟังก์ชันคำนวณแฟกทอรีล`<br>
`ตรวจสอบเงื่อนไขว่าถ้า num เป็น 0 จะคืนค่า 1 ทันที`<br>
`หากไม่ใช่ จะใช้ลูป for วนซ้ำตั้งแต่ 1 ถึง num (รวม num ด้วยผ่าน 1..=num) เพื่อนำค่ามาคูณสะสมในตัวแปร result จนครบ (1 x 2 x 3 x 4 x 5 = 120) แล้วคืนค่าผลลัพธ์กลับไป`

---

### Example 2 — `[discount]`

**Purpose:** `ทำให้รู้ว่า Function สร้าง Function ได้`

```rust
use std::io;
#[derive(Debug)]
struct Product {
    name: String,
    price: f64,
}

fn create_discount(discount: f64) -> impl Fn(f64) -> f64 {
    move |price| price * (1.0 - discount)
}
fn calculate(
    products: &[Product],
    operation: impl Fn(f64) -> f64,
) {
    for product in products {
        let new_price = operation(product.price);

        println!(
            "{} : {:.2} -> {:.2}",
            product.name,
            product.price,
            new_price
        );
    }
}
fn input(message: &str) -> String {
    let mut value: String = String::new();
    println!("{}", message);
    io::stdin().read_line(&mut value).unwrap();
    value.trim().to_string()
}

fn main() {
    let name1 = input("Product 1 name:");
    let price1: f64 = input("Product 1 price:").parse().unwrap();
    let name2 = input("Product 2 name:");
    let price2: f64 = input("Product 2 price:").parse().unwrap();
    let name3 = input("Product 3 name:");
    let price3: f64 = input("Product 3 price:").parse().unwrap();

    let discount: f64 = input("Discount (%):").parse().unwrap();

    let products = [
        Product {
            name: name1,
            price: price1,
        },
        Product {
            name: name2,
            price: price2,
        },
        Product {
            name: name3,
            price: price3,
        },
    ];

    let discount_fn = create_discount(discount / 100.0);

    calculate(&products, discount_fn);
}


//let mut input = String::new();
//io::stdin().read_line(&mut input).unwrap();
//let age: i32 = input.trim().parse().unwrap();


```
**input**
Product 1 name:
iphone 
Product 1 price:
29990
Product 2 name:
ipad 
Product 2 price:
27900
Product 3 name:
mac
Product 3 price:
44900
Discount (%):
10
**Output**
iphone : 29990.00 -> 26991.00
ipad : 27900.00 -> 25110.00
mac : 44900.00 -> 40410.00
```text
[expected output]
```

**Explanation**

`operation = ฟังก์ชันที่ส่งเข้ามาเป็น Parameter `<br>
`Fn(f64) = รับค่าตัวเลขชนิด f64`<br>
`-> f64 = คืนค่าตัวเลขชนิด f64`<br>

---
## 7. Common Mistakes

### Mistake 1 — Function Overloading

**Problem**

บางครั้งอาจเกิดการสับสนในการใช้ และคิดว่าภาษา rust สามารถทำ `Function Overloading` ได้ (การสร้างฟังก์ชันชื่อเดียวกัน แต่รับ Parameter หรือมี Type ต่างกัน)  แต่ในความจริงเป็นแล้ว rust ไม่รองรับการทำ Function Overloading โดยตรง

**Incorrect Code**

```rust
fn add(x:i32 , y:i32) -> i32{
    x + y
}

fn add(x: f64, y: f64) -> f64 {
    x + y
}

fn main() {
    let sum1 = add(2,5);
    let sum2 = add(2.2,5.5);
    println!("sum1 = {}",sum1);
    println!("sum2 = {}",sum2);
}
```

**Correct Code**

```rust
fn add_i32(x:i32 , y:i32) -> i32{
    x + y
}

fn add_f64(x: f64, y: f64) -> f64 {
    x + y
}

fn main() {
    let sum1 = add_i32(2,5);
    let sum2 = add_f64(2.2,5.5);
    println!("sum1 = {}",sum1); // sum1 = 7
    println!("sum2 = {}",sum2); // sum2 = 7.7
}
```

**Why?**

* เพื่อป้องกันความสับสนของ `Type Inference` : หากมี Overloading อาจทำให้ Compiler สับสนหรือคาดเดาประเภทข้อมูลผิดจากที่ผู้เขียนตั้งใจ

* แนวทางใกล้เคียง : ใช้ `Generics` หรือ `Traits` แทน Function Overloading 

---

### Mistake 2 — Default Parameter

**Problem**

ภาษา rust ไม่รองรับการทำ `default parameter`(การกำหนดค่าเริ่มให้กับฟังก์ชัน)

**Incorrect Code**

```rust
fn greet(name: &str, msg: &str = "Hello") {
    println!("{}, {}!",msg,name);
}
fn main() {
    greet("Alice", None);               
    greet("Bob", Some("Good morning")); 
}
```

**Correct Code**

```rust
fn greet(name: &str, msg: Option<&str>) {
    let greeting = match msg {
        Some(msg) => msg,
        None => "Hello",
    };
    println!("{}, {}!", greeting, name);
}
fn main() {
    greet("Alice", None); // Hello, Alice!    
    greet("Bob", Some("Good morning")); // Good morning, Bob!
}
```

**Why?**

* เพื่อความชัดเจน (Explicit over Implicit): Rust เน้นให้ผู้อ่านโค้ดมองเห็นสิ่งที่เกิดขึ้นอย่างชัดเจนที่สุด โดยไม่มีระบบซ่อนการทำงานอยู่เบื้องหลัง

* แนวทางใกล้เคียง : ใช้ `Option<T>` หรือ `Default Trait` แทน Default Parameters

---

## 8. Exercises

### Exercise 1 — Is_Even

**Problem**

จงเขียนฟังก์ชันชื่อ `is_even` ที่รับตัวเลขจำนวนเต็ม `(i32)` เข้ามา 1 ตัว แล้วทำการตรวจสอบว่าตัวเลขนั้นเป็น เลขคู่ (Even Number) หรือไม่
* หากเป็นเลขคู่ ให้พิมพ์คำว่า "`True`"
* หากเป็นเลขคี่ ให้พิมพ์คำว่า "`False`"

**Hint**

* สามารถตรวจสอบเลขคู่ โดยใช้ตัวดำเนินการ Modulo (%)

**Solution**

```rust
fn is_even(number: i32){
    if number % 2 == 0 {
        println!("True");
    } else {
        println!("False");
    }
}
fn main() {
    let num1 = 4;
    let num2 = 7;

    is_even(num1); // True
    is_even(num2); // False
}
```
**Explanation**

* `fn is_even(number: i32)`  : สร้างฟังก์ชันชื่อ `is_even`  ที่รอรับค่าตัวเลขจำนวนเต็ม `(i32)` เข้ามาผ่านพารามิเตอร์ชื่อ `number`
* `if number % 2 == 0`  : นำเลขมาหารด้วย 2 ถ้าหารลงตัว เศษเป็น 0 หมายความว่าตัวเลขนั้นเป็นเลขคู่ ให้พิมพ์ `"True"` 
* `else`  : ถ้าหารไม่ลงตัว หมายความว่าตัวเลขนั้นเป็นเลขคี่ ให้พิมพ์ `"False"` 
* `main()`  : มีการประกาศตัวแปรที่เป็นตัวเลข และส่งตัวแปรไปเช็ค โดยส่ง 4 ได้ผลลัพธ์เป็น True และส่ง 7 ไปเช็ค ได้ผลลัพธ์เป็น `False`

---
### Exercise 2 — Grade_Check

**Problem**

จงเขียนฟังก์ชันชื่อ `grade_check` โดยรับพารามิเตอร์เป็นตัวเลขจำนวนเต็ม  `score (i32)` โดยคะแนนที่รับมาจะไม่เป็นค่าติดลบ หรือมากกว่า 100  จากนั้นจะตัดเกรดโดยใช้เกณฑ์ว่า ถ้าได้ไม่ถึง 40 คะแนนได้เกรดเป็น Fail, ได้ถึง 40 แต่ไม่ถึง 80 คะแนนได้เกรดเป็น Pass, และถ้าได้
ถึง 80 คะแนน จะได้เกรดเป็น Excellent

**Hint**

* ฟังก์ชันนี้ไม่ต้องคืนค่า `(ไม่ต้องใส่ -> ...)` ใช้ `println!` แสดงข้อความในฟังก์ชันได้เลย

**Solution**

```rust
fn grade_check(score: i32){
    if score >= 80 {
        println!("Exellent");
    }else if score >= 40 {
        println!("Pass");
    } else {
        println!("Fail");
    }
}
fn main() {
    grade_check(39); // Fail
    grade_check(40); // Pass
    grade_check(90); // Exellent
}
```

**Explanation**

* `fn grade_check(score: i32)`
    * รับพารามิเตอร์ `score` ชนิดจำนวนเต็ม `i32`
    * ไม่มี `-> ...` แปลว่าฟังก์ชันนี้ไม่คืนค่า ทำหน้าที่แค่แสดงข้อความออกหน้าจอ

* `if score >= 80` : ถ้าคะแนนมากกว่าหรือเท่ากับ 80 ให้แสดง `Excellent` 

* `else if score >= 40` : ถ้า`score` น้อยกว่า 80 และ `>= 40` ให้แสดง `Pass`

* `else` : กรณีที่เหลือทั้งหมด คือ `score` น้อยกว่า 40 ให้แสดง `Fail`

* `grade_check(...)` ส่งค่าตัวเลขไปยังฟังก์ชัน จะแสดงผลออกมาเป็น `Excellent` `Pass` และ `Fail` ตามเงื่อนไขที่เรากำหนดไว้

---

