fn main() {
  let idade:u8 = 18;
  let responsavel_autorizou:bool = true;

  if idade >= 18 {
    println!("Você pode entrar na festa");
  } else if idade > 16 && responsavel_autorizou   {
    println!("Você pode entrar na festa, mas o responsável autorizou");
  } else {
    println!("Você não pode entrar na festa");
  }
  
  let condicao = if idade >= 18 { "maior de idade"} else { "menor de idade"};

  println!("Você é {}", condicao);
}