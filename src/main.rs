fn main() {
    println!("Hello, 123!");


    let mut database: Vec<User> =Vec::new();


    register("Özlem".to_string(), "Özlem@gmail.com".to_string(), "özlem123".to_string(), &mut database);

    login("Özlem@gmail.com".to_string(), "özlem13".to_string(), &mut database);

}






#[derive(Clone,Debug)]
struct User {

    name:String,
    email:String,
    password:String

}



fn login(email:String,password:String,database :&mut Vec<User>) {

    for user in database  {

        if email == user.email && password==user.password {

            println!("Giriş Başarılı")
            
        }
        else {
            println!("Şifre yada email hatalı ")
        }
        
    }

    
}


fn register(name:String,email:String,password:String,database :&mut Vec<User>) {

    if !email.contains('@') {

        println!("Yanlış email girdiniz")
        
    }
    else {
        let user1=User{
            name:name,
            email:email,
            password:password

        };


        database.push(user1);

        println!("Kaydınız başarılı  ")


    }
    
}
