fn main() {
  let multiplicador:u8 = 5;

  let mut contador:u8 = 0;

  while contador < 10 {
    contador += 1; 

    if contador == 5 {
      println!("Contador é 5, pulando para o próximo");
      continue;
    }

    println!("{} * {} = {}", multiplicador, contador, multiplicador * contador);
  } 

  contador = 0;

  loop {
    contador += 1;
    println!("{} * {} = {}", multiplicador, contador, multiplicador * contador);
    
    if contador == 10 {
      break;
    }
  }

}