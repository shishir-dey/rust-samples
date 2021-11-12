fn main() {
    struct Employee {
        name: String,
        age: u8,
        salary: u32,
    }
    
    let e1 = Employee { name: "Abc".to_string(), age: 21, salary: 30000, };
    println!("Employee e1\nName: {}\nAge: {}\nSalary: ${}", e1.name, e1.age, e1.salary);
}
