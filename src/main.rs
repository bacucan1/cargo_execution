use std::io;
use rand::Rng;
struct Pregunta {
    texto: String,
    respuestas: Vec<Respuesta>,
}

struct Respuesta {
    texto: String,
    valor: bool,
}

fn main() {
    let mut cuenta : Vec<i32> = vec![];
    println!("Bienvenido, quieres jugar rust challenge:");
    println!("Por fa digita tu nombre para comenzar:");
    let mut nombre=tomaNombre();
    let mut cur_res : i32 = 0;
    let mut sum = 0;
    for i in 0..5 {
        cur_res=Eva_facil();
        cuenta.push(cur_res);
        sum=sum+cur_res;
    }
    println!("puntaje: {}",sum);
    for i in 0..5 {
        cur_res=Eva_media();
        cuenta.push(cur_res);
        sum=sum+cur_res;
    }
    println!("puntaje: {}",sum);
    for i in 0..5 {
        cur_res=Eva_dificil();
        cuenta.push(cur_res);
        sum=sum+cur_res;
    }
    println!("puntaje: {}",sum);
    for i in 0..5 {
        cur_res=Eva_extremo();
        cuenta.push(cur_res);
        sum=sum+cur_res;
    }
    println!("puntaje: {}",sum);
    
}
fn tomaNombre()->String{
    let mut c = String::new();
    io::stdin()
        .read_line(&mut c)
        .expect("failed to read input.");
    return c;

}
fn Eva_facil() -> i32{
    let pregunta_1 = Pregunta { texto: String::from("Cuál de las siguientes características de Rust es falsa"),
    respuestas: vec![
    Respuesta{texto:String::from("Seguridad de memoria"), valor: false}, 
    Respuesta{texto:String::from("Rendimiento"), valor: false},
    Respuesta{texto:String::from("Lenguaje de alto nivel"), valor: true},
    Respuesta{texto:String::from("Abstracciones de alto nivel"), valor: false},
    ]};
    let pregunta_2 = Pregunta { texto: String::from("Rust tiene una sintaxis similar a:"),
    respuestas: vec![
    Respuesta{texto:String::from("Java"), valor: false}, 
    Respuesta{texto:String::from("Python"), valor: false},
    Respuesta{texto:String::from("C++"), valor: true},
    Respuesta{texto:String::from("Solidity"), valor: false},
    ]};
    let pregunta_3 = Pregunta { texto: String::from("Qué empresa desarrolló Rust"),
    respuestas: vec![
    Respuesta{texto:String::from("Microsoft"), valor: false}, 
    Respuesta{texto:String::from("Mozilla"), valor: true},
    Respuesta{texto:String::from("Google"), valor: false},
    Respuesta{texto:String::from("Brave"), valor: false},
    ]};
    let pregunta_4 = Pregunta { texto: String::from("Cargo es"),
    respuestas: vec![
    Respuesta{texto:String::from("Un prograama de instalación de Rust"), valor: false}, 
    Respuesta{texto:String::from("Un sistema de construcción y administración de paquetes"), valor: true},
    Respuesta{texto:String::from("Un compilador"), valor: false},
    Respuesta{texto:String::from("Una estructura de sintaxis"), valor: false},
    ]}; 
    let pregunta_5 = Pregunta { texto: String::from("Con qué comando validamos la instalación de Rust"),
    respuestas: vec![
    Respuesta{texto:String::from("uprust --vesrion"), valor: false}, 
    Respuesta{texto:String::from("uprust -version"), valor: false},
    Respuesta{texto:String::from("--ersion rustup"), valor: false},
    Respuesta{texto:String::from("rustup --version"), valor: true},
    ]};    
    let preguntas =vec!(pregunta_1, pregunta_2, pregunta_3, pregunta_4, pregunta_5);
    let mut rng = rand::thread_rng();
    let len = preguntas.len() as i32;
    let x : usize = rng.gen_range(0..len) as usize;
    let mut current = &preguntas[x];    
    println!("{}",current.texto);
    let mut opcion = String::new();
    let mut j=0;
    for i in &current.respuestas{
        
        println!("{}. {}",j, current.respuestas[j].texto);
        j=j+1
    }
    println!("Escoge tu respuesta");
    io::stdin()
        .read_line(&mut opcion)
        .expect("failed to read input.");
    let opcion = opcion.trim().parse::<usize>().expect("invalid input");
    let mut res = current.respuestas[opcion].valor;
    let mut z= 0;
    if res == true{z=2} else{z=0}
    return z;
}

fn Eva_media() -> i32{
    let pregunta_1 = Pregunta { texto: String::from("Cuál de las siguientes funciones está bien escrita"),
    respuestas: vec![
    Respuesta{texto:String::from("function tomaNombre -> ()"), valor: true}, 
    Respuesta{texto:String::from("funtion tomaNombre => ()"), valor: false},
    Respuesta{texto:String::from("function tomaNombre () => {}"), valor: false},
    Respuesta{texto:String::from("function tomaNombre () -> {}"), valor: false},
    ]};
    let pregunta_2 = Pregunta { texto: String::from("Para crear referencias en Rust, que van seguidas del valor al que se quiere hacer referencia, se utiliza el operador"),
    respuestas: vec![
    Respuesta{texto:String::from("&"), valor: true}, 
    Respuesta{texto:String::from("Xor"), valor: false},
    Respuesta{texto:String::from(">="), valor: false},
    Respuesta{texto:String::from("^"), valor: false},
    ]};
    let pregunta_3 = Pregunta { texto: String::from("Validar coincidencias de patrones y tomar decisiones en función de los diferentes casos que pueden ocurrir."),
    respuestas: vec![
    Respuesta{texto:String::from("Ejecutar código basado en una condición y/o de repetir código mientras se cumpla una condición"), valor: false}, 
    Respuesta{texto:String::from("Crear condicionales "), valor: false},
    Respuesta{texto:String::from("Iterar sobre una secuencia de elementos"), valor: false},
    Respuesta{texto:String::from("Validar coincidencias de patrones y tomar decisiones en función de los diferentes casos que pueden ocurrir."), valor: true},
    ]};
    let pregunta_4 = Pregunta { texto: String::from("Para qué sirve la programación en Rust"),
    respuestas: vec![
    Respuesta{texto:String::from("Crear páginas web interactivas y dinámicas"), valor: false}, 
    Respuesta{texto:String::from("Crear aplicaciones, sitios web, software, y automatizar tareas"), valor: false},
    Respuesta{texto:String::from("Crear contratos inteligentes para plataformas de blockchain, como Ethereum"), valor: false},
    Respuesta{texto:String::from("Desarrollar sistemas operativos"), valor: true},
    ]}; 
    let pregunta_5 = Pregunta { texto: String::from("Los campos de una estructura en Rust se encierran en"),
    respuestas: vec![
    Respuesta{texto:String::from("Llaves"), valor: true}, 
    Respuesta{texto:String::from("Corchetes"), valor: false},
    Respuesta{texto:String::from("Paréntesis"), valor: false},
    Respuesta{texto:String::from("Llave y paréntesis"), valor: false},
    ]};    
    let preguntas =vec!(pregunta_1, pregunta_2, pregunta_3, pregunta_4, pregunta_5);
    let mut rng = rand::thread_rng();
    let len = preguntas.len() as i32;
    let x : usize = rng.gen_range(0..len) as usize;
    let mut current = &preguntas[x];    
    println!("{}",current.texto);
    let mut opcion = String::new();
    let mut j=0;
    for i in &current.respuestas{
        
        println!("{}. {}",j, current.respuestas[j].texto);
        j=j+1
    }
    println!("Escoge tu respuesta");
    io::stdin()
        .read_line(&mut opcion)
        .expect("failed to read input.");
    let opcion = opcion.trim().parse::<usize>().expect("invalid input");
    let mut res = current.respuestas[opcion].valor;
    let mut z= 0;
    if res == true{z=4} else{z=-1}
    return z;
}

fn Eva_dificil() -> i32{
    let pregunta_1 = Pregunta { texto: String::from("El patrón match: match en Rust"),
    respuestas: vec![
    Respuesta{texto:String::from("Descompone estructuras complejas, como tuplas, estructuras (structs) o enumeraciones (enums), directamente en los parámetros de una función."), valor: false}, 
    Respuesta{texto:String::from("Se utiliza para trabajar con referencias a datos en lugar de los datos en sí mismos"), valor: false},
    Respuesta{texto:String::from("Compara un valor contra varios patrones y ejecuta el código asociado al primer patrón que coincida."), valor: true},
    Respuesta{texto:String::from("Permite desestructurar una tupla directamente en los parámetros de una función."), valor: false},
    ]};
    let pregunta_2 = Pregunta { texto: String::from("Cuál se las siguientes es la sintaxis básica para definir una cadena en Rust"),
    respuestas: vec![
    Respuesta{texto:String::from("let <nombre_de_la_variable> = <valor_de_la_variable>;"), valor: true}, 
    Respuesta{texto:String::from("let <nombre_de_la_variable> = <valor_de_la_variable>"), valor: false},
    Respuesta{texto:String::from("let <nombre-de-la-variable> = <valorde-de-la-variable>;"), valor: false},
    Respuesta{texto:String::from("let <nombre de la variable> = <valor de lavariable>;"), valor: false},
    ]};
    let pregunta_3 = Pregunta { texto: String::from("Cual de las siguientes expresiones es la indicada para especificar una cadena de texto inmutable en Rust"),
    respuestas: vec![
    Respuesta{texto:String::from("let cadena : #string"), valor: false}, 
    Respuesta{texto:String::from("let cadena : : &string"), valor: false},
    Respuesta{texto:String::from("let cadena : &str"), valor: true},
    Respuesta{texto:String::from("let cadena = str"), valor: false},
    ]};
    let pregunta_4 = Pregunta { texto: String::from("La ubicacion de un valor en el stack en Rust se da de la siguiente manera"),
    respuestas: vec![
    Respuesta{texto:String::from("Almacena valores en el orden en que son recibidos y los elimina en el mismo orden"), valor: false}, 
    Respuesta{texto:String::from("Almacena valores en el orden en que son recibidos y los elimina en orden inverso"), valor: true},
    Respuesta{texto:String::from("Almacena valores en el espacio que el administrador de memoria le asigne"), valor: false},
    Respuesta{texto:String::from("Almacena valores en el orden inverso en que son recibidos y los elimina en en el mismo orden"), valor: false},
    ]}; 
    let pregunta_5 = Pregunta { texto: String::from("Cual de las siguientes opciones no forma parte de un String en Rust"),
    respuestas: vec![
    Respuesta{texto:String::from("Un puntero a la memoria que contiene el contenido de la cadena"), valor: false}, 
    Respuesta{texto:String::from("Longitud"), valor: false},
    Respuesta{texto:String::from("Capacidad"), valor: false},
    Respuesta{texto:String::from("Uso de parentesis"), valor: true},
    ]};    
    let preguntas =vec!(pregunta_1, pregunta_2, pregunta_3, pregunta_4, pregunta_5);
    let mut rng = rand::thread_rng();
    let len = preguntas.len() as i32;
    let x : usize = rng.gen_range(0..len) as usize;
    let mut current = &preguntas[x];    
    println!("{}",current.texto);
    let mut opcion = String::new();
    let mut j=0;
    for i in &current.respuestas{
        
        println!("{}. {}",j, current.respuestas[j].texto);
        j=j+1
    }
    println!("Escoge tu respuesta");
    io::stdin()
        .read_line(&mut opcion)
        .expect("failed to read input.");
    let opcion = opcion.trim().parse::<usize>().expect("invalid input");
    let mut res = current.respuestas[opcion].valor;
    let mut z= 0;
    if res == true{z=5} else{z=-2}
    return z;
}
fn Eva_extremo() -> i32{
    let pregunta_1 = Pregunta { texto: String::from("Cual de las siguientes opciones no es correcta al decir que match en Rust tiene que ser exhaustivo"),
    respuestas: vec![
    Respuesta{texto:String::from("Ayuda en la refactorizacion"), valor: false}, 
    Respuesta{texto:String::from("Ayuda en la claridad"), valor: false},
    Respuesta{texto:String::from("Eliminan las fuentes de error"), valor: false},
    Respuesta{texto:String::from("Simplifica el algoritmo de interferencia"), valor: true},
    ]};
    let pregunta_2 = Pregunta { texto: String::from("Cual de las siguientes opciones no es correcta
acerca de los slices en Rust"),
    respuestas: vec![
    Respuesta{texto:String::from("Un slice es una especie de referencia, por lo que no posee la propiedad de los datos."), valor: false}, 
    Respuesta{texto:String::from("Los slices proporcionan una vista temporal e inmutable de los datos sin tomar propiedad"), valor: false},
    Respuesta{texto:String::from("Son una forma de tomar prestado un valor sin adquirir la propiedad de este"), valor: true},
    Respuesta{texto:String::from("Los slices se componen de un puntero y una longitud"), valor: false},
    ]};
    let pregunta_3 = Pregunta { texto: String::from("Cual de las siguientes estructuras esta correctamente escrita"),
    respuestas: vec![
    Respuesta{texto:String::from("struct PersonaUno {
    nombre: String,
    equipo: String,
}"), valor: true}, 
    Respuesta{texto:String::from("struct PersonaUno [
    nombre: String,
    equipo: String,
    ]"), valor: false},
    Respuesta{texto:String::from("struct PersonaUno {
    nombre: : String,
    equipo: : String,
    }"), valor: false},
    Respuesta{texto:String::from("struct PersonaUno {
    nombre: String;,
    equipo: String;
    }"), valor: false},
    ]};
    let pregunta_4 = Pregunta { texto: String::from("Cual es el operador para la operacion: AND bit a bit y asignación"),
    respuestas: vec![
    Respuesta{texto:String::from("%="), valor: false}, 
    Respuesta{texto:String::from("&="), valor: true},
    Respuesta{texto:String::from("^="), valor: false},
    Respuesta{texto:String::from(">>="), valor: false},
    ]}; 
    let pregunta_5 = Pregunta { texto: String::from("Cuál de las siguientes afirmaciones no es cierta en cuanto a Ownership y Borrowing en Rust"),
    respuestas: vec![
    Respuesta{texto:String::from("Iintroduce un sistema de propiedad y préstamo único para gestionar la memoria de forma segura"), valor: false}, 
    Respuesta{texto:String::from("Cada valor en Rust tiene un único propietario, y los valores se pasan entre las partes del código mediante préstamos"), valor: false},
    Respuesta{texto:String::from("Este sistema garantiza que no haya punteros colgantes ni doble liberación de memoria"), valor: false},
    Respuesta{texto:String::from("Permite validar coincidencias de patrones y tomar decisiones en función de los diferentes casos que pueden ocurrir"), valor: true},
    ]};    
    let preguntas =vec!(pregunta_1, pregunta_2, pregunta_3, pregunta_4, pregunta_5);
    let mut rng = rand::thread_rng();
    let len = preguntas.len() as i32;
    let x : usize = rng.gen_range(0..len) as usize;
    let mut current = &preguntas[x];    
    println!("{}",current.texto);
    let mut opcion = String::new();
    let mut j=0;
    for i in &current.respuestas{
        
        println!("{}. {}",j, current.respuestas[j].texto);
        j=j+1
    }
    println!("Escoge tu respuesta");
    io::stdin()
        .read_line(&mut opcion)
        .expect("failed to read input.");
    let opcion = opcion.trim().parse::<usize>().expect("invalid input");
    let mut res = current.respuestas[opcion].valor;
    let mut z= 0;
    if res == true{z=7} else{z=-4}
    return z;
}