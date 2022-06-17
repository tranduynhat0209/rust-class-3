use std::collections::HashMap;

fn main() {
    // Basic School - grade type is u32
    let mut school = School::new();

    school.add(4, "Alice");
    school.add(4, "Bob");
    school.add(3, "Charlie");
    school.add(10, "Steve");

    let grades = school.grades();
    println!("{:?}", grades);

    let students = school.grade(4);
    println!("{:?}", students);

    // Advanced School - grade type is generic type

    // Case 1: String grade
    let mut school = AdvancedSchool::<String>::new();
    school.add(String::from("A"), "Alice");
    school.add(String::from("A"), "Bob");
    school.add(String::from("B+"), "Charlie");
    school.add(String::from("C"), "Steve");

    let grades = school.grades();
    println!("{:?}", grades);

    let students = school.grade(String::from("A"));
    println!("{:?}", students);

    // Case 2: f32 grade
    let mut school = AdvancedSchool::<f32>::new();
    school.add(1.23, "Alice");
    school.add(2.33, "Bob");
    school.add(9.5, "Charlie");
    school.add(2.33, "Steve");

    let grades = school.grades();
    println!("{:?}", grades);

    let students = school.grade(2.33);
    println!("{:?}", students);
}
// Bài tập
// Cho ngữ cảnh như sau : Một ngôi trường cần lập danh
//sách thông tin sinh viên bao gồm tên sinh viên và điểm của sinh viên đó.
// với mục đích thống kê kiểm tra giáo dục của ngôi trường này

/*-----------------------------*/
// Gợi ý:
// Định nghĩa bằng struct, mọi người nên sử dụng HashMap
// Tại sao lại sử dụng HashMap và ko phải Vec
//https://doc.rust-lang.org/std/collections/struct.HashMap.html
// struct School {
//     students: HashMap<String, u32>
// }

// trong trường hợp này thì String : tên của sinh viên đó
// u32 là điểm số

// Một số yêu cầu như sau:

/*-----------------------------*/
//0. Tạo 1 function new() khởi tạo rỗng ban đầu cho danh sách

/*-----------------------------*/
//1. Có thể thêm thông tin của sinh viên gồm có tên và điểm
// Ví dụ: thêm tên "Alice" có 7 điểm, thêm tên "Bob" có 2 điểm, ...
// Gợi ý : định nghĩa hàm "add" implement cho Struct

/*-----------------------------*/
//2. Liệt kê các điểm số hiện tại mà trường đã cập nhập
// ví dụ :danh sách hiện tại gồm có [(Alice, 10), (Bob,4)]
//trả về là [4,10] (điểm số nên tăng dần và ko có duplicate)
// ví dụ: [(Alice, 10), (Bob,4), (Steve,4)] -> [4,10]

/*-----------------------------*/
//3. Liệt kê danh sách các học sinh có cùng 1 điểm số
// ví dụ: hiện tại danh sách gồm có (Alice, 3), (Bob, 10), (Charlie,3)
// liệt kê danh sách học sinh có cùng 3 điểm : [Alice, Charlie]

// Yêu cầu trả về: danh sách sinh viên là alphabet theo tên

// Gợi ý:
// ví dụ : Ban đầu [(Alice, 10), (Bob,2), (Eve,4), (Long,2)] -> [(Bob, 2), (Long,2), (Eve,4), (Alice,10)]
//định nghĩa hàm "find_student" có tham số là điểm số -> trả về danh sách các sinh viên có cùng điểm số mong muốn

// Các bạn phải vuợt qua một số test case như sau :

/*-----------------------------*/
//Test case 1: Khởi tạo đầu tiên danh sách phải rỗng

/*-----------------------------*/

// Test case 2:
//Thêm sinh viên có tên "Lee" với điểm số là 2
// thì tất cả các điểm số hiện có của trường là 2
//nếu thêm sinh viên khác "Nancy" với điểm số là 3
//thì các điểm số hiện tại là [2,3]

/*-----------------------------*/

// Test case 3:
// Giả sử danh sách hiện tại : [(Bob, 4), (Alice,4), (Tom,5)]
// với điểm số 4 thì ta có sinh viên nào: -> [Alice, Bob] not [Bob ,Alice]
// vì cần tên theo alphabet

/*-----------------------------*/
// Nếu mọi người làm xong rùi thì có thể làm advance hơn bằng cách
// sử dụng Generic type cho điểm số không nhất thiết phải U32 nữa mà có thể "A+", "B+" chẳng hạn (string)
/*-----------------------------*/

// Sườn thông tin cho mọi người dễ làm
trait SchoolTrait<T> {
    fn new() -> Self;
    fn add(&mut self, grade: T, student: &str);
    fn grades(&self) -> Vec<T>;
    fn grade(&self, grade: T) -> Vec<String>;
}
pub struct School {
    // !TODO
    students: HashMap<String, u32>,
}

impl SchoolTrait<u32> for School {
    fn new() -> School {
        Self {
            students: HashMap::new(),
        }
    }

    fn add(&mut self, grade: u32, student: &str) {
        self.students.insert(String::from(student), grade);
    }

    fn grades(&self) -> Vec<u32> {
        let mut result: Vec<u32> = Vec::new();

        for grade in self.students.values(){
            if !result.contains(grade){
                result.push(*grade);
            }
        }
        result.sort_by(|a, b| a.partial_cmp(b).unwrap());
        result
    }

    fn grade(&self, grade: u32) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();

        for student in self.students.iter(){
            if *student.1 == grade{
                result.push(student.0.to_string());
            }
        }
        result.sort();
        result
    }
}

pub struct AdvancedSchool<T: PartialEq + PartialOrd + Clone>{
    students: HashMap<String, T>
}

impl<T: PartialEq + PartialOrd + Clone> SchoolTrait<T> for AdvancedSchool<T> {
    fn new() -> Self{
        Self { students: HashMap::new() }
    }
    fn add(&mut self, grade: T, student: &str){
        self.students.insert(String::from(student), grade);
    }
    fn grades(&self) -> Vec<T>{
        let mut result: Vec<T> = Vec::new();

        for grade in self.students.values(){
            if !result.contains(grade){
                result.push(grade.clone());
            }
        }
        result.sort_by(|a, b| a.partial_cmp(b).unwrap());
        result
    }
    fn grade(&self, grade: T) -> Vec<String>{
        let mut result: Vec<String> = Vec::new();

        for student in self.students.iter(){
            if *student.1 == grade{
                result.push(student.0.to_string());
            }
        }
        result.sort();
        result
    }
}