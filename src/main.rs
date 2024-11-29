#![allow(dead_code)]

mod utils;

use std::{fmt, mem};

//Debug
#[derive(Debug)]
struct Person<'a>{
    name: &'a str,
    age:u8
}

//Debuging with fmt:Display
#[derive(Debug)]
struct  Minmax(f64, f64);

impl fmt::Display for Minmax {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}


fn main(){
    println!("{} days", 31);

    //Argument with positional
    println!("Name {0} email {1}", "Aldo Ratmawan", "aldo.ratmawan9999@gmai.com");

    //As can Name Arguments
    println!("Saya adalah {name} email saya {email} tempa tinggal saya {address} ", name="Aldo Ratmawan", email="aldo1909@gmail.com", address="Waylayap")
}

#[test]
fn  test_person_debug() {
    let name = "Aldo Ratmawan";
    let age = 20;

    let all = Person {name, age };

    println!("{:?}", all);
}

//Primitive type Variable
#[test]
fn scalar_types(){
    let boolean: bool = true;

    let mut  float_i64 : f64 = 100.809;
    let an_integer = 1005i32 * 19;
    float_i64 = 909.08;

    println!("{}", boolean);
    println!("float i64 : {}", float_i64);
    println!("Sufflix annotation : {}", 1e6)
}


fn reverseTuples(tupls: (i32, f64)) -> (f64, i32){
    let (int, flt) = tupls;
    (flt, int)
}

#[derive(Debug)]
struct Matrix(i32, f32, bool, u32);

#[test]
fn test_tuples(){
    let tupls:(i32, f64) = (80129 , 100.30);

    let long_tuple: ((i32, f64, bool, char), (i32, f64, bool)) = ((100i32, 30.5f64,true, 'a'), (10, 40.5, false)); // tuple members

    println!("The reversed pair is {:?} " , reverseTuples(tupls));
    println!("the Tuple long : {:#?}", long_tuple);

    let matrix = Matrix(10, 80.99, false, 90);
    println!("{:?}", matrix);
}

fn analize_slice(arr: &[i32]){
    println!("First element: {}", arr[0]);
    println!("Arras length: {}", arr.len())
}

#[test]
fn array_and_slice(){

    let my_arr :[i32; 100] = [800; 100];

    println!("{:?}", my_arr);
    println!("length: {}", my_arr.len());

    // my_arr.join(&[100, 200, 300]);

    my_arr.map(|val|{
        println!("{}", val / 2)
    });

    //arrays are stack allocated
    print!("Array : {} bytes", mem::size_of_val(&my_arr));
    analize_slice(&my_arr);
    analize_slice(&my_arr[1..10]);

    //for iteration loop

    for i in 0..my_arr.len() + 1{
        match my_arr.get(i) {
            Some(xval) => println!("{}, {}", i, xval),
            None => println!("slowdown {}", i)
        }
    }
}

//Custum Type Struct

#[derive(Debug)]
struct Data{
   data:[i32; 10]
}

#[derive(Debug)]
struct  PersonAdd{
    name    : String,
    email   : String,
    address : String,
    data    : Data,
}

#[test]
fn type_struct(){

    let name    = String::from("Aldo Ratmawan");
    let email   = String::from("aldo.ratmawna999@gmail.com");
    let address = String::from("bandar lampung");
    let data      = Data {data: [100; 10]};

    let person = PersonAdd {name, email, address, data};
    
    println!("{:?}",  person)
}

//Enum

enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPres(char),
    Paste(String),
    Click{ x: i64, y: i64 }
}

enum Math{
    Add,
    SubStract
}

enum Stage {
    Beginer,
    Advanced
}

impl  Math {
    fn run(&self, x:i32, y:i32) -> i32{
        match self {
            Self::Add => x + y,
            Self::SubStract => x - y
        }
    }
}

fn inspect(ev: WebEvent){   
    match ev {
        WebEvent::PageLoad => println!("Page Load Web"),
        WebEvent::Paste(s) => println!("Ctrl + V : {}", s),
        WebEvent::Click {x,y } => {
            println!("clicked x:{} y:{}", x, y)
        },
        WebEvent::KeyPres(k) => println!("Press Key: {}", k),
        WebEvent::PageUnload => println!("Page Onloaded")
    }
}

fn staged(stage: Stage){

    match stage {
        Stage::Advanced => { println!("Advanced") },
        Stage::Beginer  => { println!("Beginer") }
    }
}

#[test]
fn enum_test(){
    
    let pressed  = WebEvent::KeyPres('x');
    let page_load = WebEvent::PageLoad;

    //instance enum implementation
    let add_operation = Math::Add;
    let substract = Math::SubStract;

    //use enum

    use crate::Stage::{ Advanced, Beginer };

    let stage = Beginer;

    staged(stage);

    inspect(pressed);
    inspect(page_load);

    println!("add operation : {}", add_operation.run(10, 90));
    println!("subsract operation : {}", substract.run(80, 70));
}

//struct impl traint

trait Speak {
    fn sayHello(&self);
}

struct Human;
struct Robot;

impl Speak for Human {
    fn sayHello(&self) {
        println!("I Human Bro not Robot")
    }
}

impl  Speak for Robot {
    fn sayHello(&self) {
        println!("I Robot not Human")
    }
}

#[test]
fn strut_impl_trait(){

    //create instance for struct
    let human = Human;
    let robot = Robot;

    Human.sayHello();
    human.sayHello();
    robot.sayHello();
}

//Variable Bindings
#[test]
fn variable_binding_mutability(){
    //muatability
    let _immutable_binding = 10;
    let mut mutable_binding = 100;
    
    mutable_binding = 800;
    println!("immutable: {}, mutable : {}", _immutable_binding, mutable_binding);
}

#[test]
fn scope_and_shawding(){

    let mut changed_data: i32 = 100;
    let long_live_variable = 100;
    {
        let shadow_variable = 1000;
        println!("shadow variable : {shadow_variable}");
        {
            changed_data = 1000;
            {
                println!("Hello World")
            }
        }
    };
    println!("changed data in showding : {}", changed_data)
}

//Convertion From and Into

// use std::convert::From;

#[derive(Debug)]
struct Number{
    value : i32,
    valu2: i32
}

impl From<i32> for Number {
    fn from(value: i32) -> Self {
        Number { value: value, valu2: value * 19}
    }
}

#[test]
fn convertion_from() {

    let mut num = Number::from(100);
    num.value = 100;
    println!("addNum: 1 -> {}, 2 -> {}", num.value, num.valu2)
}   


#[derive(Debug)]
struct  ABC{
    value: i32,
    value1: i32,
}

#[derive(Debug)]
struct Scale{
    x: i32,
    y: i32
}

#[derive(Debug)]
struct  ImpementatioanScale{
    top: Scale,
    bottom: Scale
}

impl  Into<ImpementatioanScale> for Scale{
    fn into(self) -> ImpementatioanScale {
        ImpementatioanScale { 
            top: Scale { x: self.x, y: self.y }, 
            bottom: Scale { x: 10, y: 100 } 
        }
    }
}

impl Into<ABC> for i32 {
    fn into(self) -> ABC {
        ABC { value:self, value1: self * 20 }
    }
}

#[test]
fn convertion_into() {

    let int = 100;
    let num: ABC = int.into();

    let scale: Scale = Scale { x: 100, y: 200 };
    let mplls : ImpementatioanScale =  scale.into();

    match mplls {
        ImpementatioanScale { 
            top, 
            bottom } => println!("Hello")
    }

    println!("My Number is : {:?}", num);
    println!("value :{} , value2:{}", num.value, num.value1)
}
//TryFrom and TryInto

#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0{
            Ok(EvenNumber(value))
        }else {
            Err(())
        }
    }
}

#[test]
fn try_fron_try_into() {
    
    //try_from
    assert_eq!(EvenNumber::try_from(100), Ok(EvenNumber(100)));

    //try_into  
    let result = 8i32.try_into();
    assert_eq!(result, Ok(EvenNumber(8)));
}

//flow and control

#[test]

fn loop_test(){

    let mut count = 0i32;
    loop {
        count += 1;

        if count >= 100{
            println!("one houndred");
            break;
        }

        if count < 99 {
            println!("{}", count);
            continue;
        }
    }
    //neting and label Loop

    let mut outer_count = 0i32;
    let mut inner_count = 0i32;

    'outer: loop {

        outer_count += 1;
        inner_count += 1;

        println!("this is loop outer data");

        'inner: loop {
            break 'outer;
        }
    }

    //return from loop
    let mut count_utable = 0i32;

    let result = loop {
        count_utable += 1;

        if count_utable >= 100{
            break count_utable / 2;
        }
    };

    println!("count utable loop result: {}", count_utable)
}


#[test]

fn while_loop(){

    let mut count = 0i32;

    while count <= 101 {
        
        if count % 15 == 0{
            println!("Fizzbuz: {}", count)
        }

        if count % 3 == 0{
            println!("fizz : {}", count)
        }

        if count % 5 ==0 {
            print!("Buzz : {}", count)
        }
        count += 1;
    }
}


#[derive(Debug)]
struct AddTG{
    name: String,
    email: String
}

#[test]

fn for_loop_in_range(){


    //example for in range
    for i in 1..=101{

        if i % 15 == 0{
            println!("Fizzbuz -> {}", i)
        }

        if i % 3 == 0{
            println!("fizz -> {}", i);
        }

        if i % 5 == 0{
            println!("Buzz -> {}", i)
        }
    }

    //example for n iterator

    let vector = vec!["Aldo", "Otong", "Budi"];

    let mut  vectorStruc: Vec<AddTG> = Vec::new();

    vectorStruc.push(AddTG { name: String::from("Aldo Ratmawan"), email: String::from("aldo.ratmawan999@gmail.com") });
    vectorStruc.push(AddTG { name: String::from("Otong"), email: String::from("otong123@gmail.com") });

    for i in vector.iter() {
        match i {
            &"Aldo" => println!("Hello Boy {}", i),
            _ => println!("Hello => {}", i)
        }
    }


    //for vector immutable
    for add in vectorStruc.iter()  {
        let kbg = add;
        match kbg {
            AddTG { name, email } => {
               if name.contains("Aldo Ratmawa"){
                    println!("Hello : {}", name);
               }else{
                println!("Guys : {}, {}", name, email)
               }
            }
        }
    }   
}

//Destructing Match struct
#[derive(Debug)]
struct Circle{
    radius: i32,
    x: i32,
    y: i32
}


#[test]
fn destructing_match_struct(){
    let instance_struct = Circle {radius: 10, x: 20, y: 90};
    match instance_struct {
        Circle { radius, x, y } => println!("println circle radius :{}, x: {}, y: {}", radius, x, y),
    }
}

#[allow(dead_code)]
enum Temparature {
    Celcius(f32),
    Fahreinheit(f32)
}

#[test]
fn guard_match(){

    let temperature = Temparature::Celcius(45.90);

    match temperature {
        Temparature::Celcius(t) if t > 30.60 => println!("Celcius is Hot : {}", t),
        Temparature::Celcius(t) => println!("Temperature Celcius {}", t),
        Temparature::Fahreinheit(t) => print!("Fahreinheit is {}", t)
    }
}

#[test]

fn if_let(){

    //if let manual
    let some : Option<i32>  = Some(200);
    let none : Option<u32>  = None;
    
    match some {
        Some(i) if i > 100 => println!("Some is > {}", i),
        Some(i) => print!("Some is {}", i),
        _ => {}
    }

    if let Some(i) = some{
        println!("{:?}", i)
    }

    if let Some(i) = none{
        println!("{}", true)
    }else {
        println!("{}", false)
    }
}

//Function
#[derive(Debug)]

struct  Point{
    x: f32,
    y: f32
}

impl Point {
    fn origin() -> Point{
        Point { x: 0.0, y: 0.0 }
    }

    fn set(x: f32, y: f32) -> Point{
        Point { x: x, y: y }
    }
}

#[derive(Debug)]
struct Rectangle{
    p1: Point,
    p2: Point
}

impl  Rectangle {

    fn area(&self) -> f32{
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;
        
        ((x1 - x2) * (y1 - y2 )).abs()
    }

    fn perimeter(&self) -> f32{
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;
        2.0 * ((x1 - x2).abs() * (y1 - y2).abs())
    }

    fn translate(&mut self, x: f32, y: f32) {

        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

#[test]

fn funtion_test(){

    //instance variable
    let origin = Point::origin();

    let rectangle = Rectangle {
        p1: Point::origin(),
        p2: Point::set(100.0, 100.0)
    };

    let some_option            = Some(origin);
    let mut some_rectangle = Some(rectangle);

    if let Some(mut arg) = some_rectangle{
        arg.translate(2.0, 5.0);
        println!("methode rectangle area: {}", arg.area());
        println!("methode rectangle perimeter : {}", arg.perimeter());
    }

    println!("{:?}", some_option);
}

//Closure Function
#[test]
fn closure_fn(){

    let cls = |arg: &str| println!("Hello Println Closure {}", arg);
    let add = |x: i32, y:i32| x + y;

    let vec = vec![10, 20, 30];
    let constain = move |need| vec.contains(need);
    println!("{}", constain(&10));
    println!("add : {:?}", add(10, 20));
}

// Closure Function as Input Parameter
fn apply<F>(f: F) where
    F : FnOnce(){
        f()
    }

fn app_applay_to_args<F>(f : F) -> i32 where 
    F: Fn(i32) -> i32{
        f(10)
    }

fn app_input_function<F: Fn(i32, f32)>(f: F){
    f(100, 200.0)
}

#[test]
fn test_closure_fn() {

    let gh = || {
        println!("Hello World 22");
    };

    let args_cls = |args: i32| 10 * args;
    println!("{}" , app_applay_to_args(args_cls));
    apply(gh);

    let input_fn = |x: i32, y:f32| {
        println!("x: [{}], y: [{}]", x, y);
    };
    app_input_function(input_fn);
}

#[derive(Debug)]
struct  Test{
    app: bool
}

//Closure Output Parameters
fn create_fn_app(args: i32) -> impl Fn(){
    let text = "Fn".to_owned();
    move || println!("{}, {}", text, args)
}

fn result_app() -> Vec<Test>{
    vec![ Test{ app: true }, Test{ app: false } ]
}

fn create_fn_mut(args: i32) -> impl FnOnce(){
    let app_text = "FnOnce".to_owned();
    move || println!("{}, {}", app_text, args)
}

#[test]
fn closure_output_parameter() {
    let create_fn = create_fn_app(100);
    create_fn();
}