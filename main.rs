use std::collections::HashMap;

fn main() {
    // Estrutura de controle 'if'
    println!("Estrutura de controle : if.");

    let x = 5;
    if x < 10 {
        println!("x é menor que 10");
        println!("\n");
    }

    // Estrutura de controle 'match'
    println!("resultadoado da estrutura match !");

    let x = 5;
    match x {
        1 => println!("x é igual a 1"),
        2 => println!("x é igual a 2"),
        _ => println!("x é diferente de 1 e 2"),
    }

    // Estrutura de controle 'for'
    println!("\n");
    println!("Estrutura de controle : for ");

    let v = vec![1, 2, 3];
    for i in v {
        println!("{}", i);
    }

    // Estrutura 'while'
    println!("Estrutura de controle : while.");

    let mut x = 5;
    while x > 0 {
        println!("{}", x);
        x = x - 1;
    }
    println!("\n");
    
    // Estrutura 'loop'
    // estrutura de controle de fluxo que executa um bloco 
    // de código indefinidamente, a menos que seja interrompida 
    // por uma instrução break.
    let mut x = 2;
    loop {
        if x > 6 {
            break;
        }
        println!("x : {}",x);
        x += 1;
    }

// 01
println!("Desafio 01");
let nome = "Olá Mundo!";
imprime_nome(nome);

// 02
println!("Desafio 02");
let nome = "Rogerio Silva";
imprime_nome(nome);

// 03
println!("Desafio 03");
let idade :i32 = 27;
imprime_idade(idade);

// 04
println!("Desafio 04");
mostra_altura(1.78);

// 05
println!("Desafio 05");
imprime_peso(73.0);

// 06
println!("Desafio 06");
calcula_idade_em_meses(27);

// 07
println!("Desafio 07");
let imc = calcula_imc(73.5,1.73);
println!("IMC :{} ",imc);

// 08
println!("Desafio 08");
imprime_numeros();

// 09
println!("Desafio 09");
imprime_amigos();

// 10
println!("Desafio 10");
imprime_frutas();

// 11
println!("Desafio 11");
minhas_informacoes();

// 12
println!("Desafio 12");
let soma = soma_numeros(4,8);
println!("Soma dos números {} ",soma);

// 13
println!("Desafio 13");
let numeros = [10, 5, 8, 12, 3];
let maior = encontrar_maior_valor(&numeros);
match maior {
    Some(valor) => println!("O maior valor é: {}", valor),
    None => println!("A lista está vazia."),
}

// 14
println!("Desafio 14");
let numeros = [10, 5, 8, 12, 3];
let menor = encontrar_menor_valor(&numeros);

match menor {
    Some(valor) => println!("O menor valor é: {}", valor),
    None => println!("A lista está vazia."),
}

// 15
println!("Desafio 15");
let numeros = [10.0, 5.0, 8.0, 12.0, 3.0];
let media = calcula_media(&numeros);
println!("A média é: {}", media);

// 16
println!("Desafio 16");
let mut numeros = vec![10, 5, 8, 12, 3];
let numeros_ordenados = sort_list(&mut numeros);
println!("Números ordenados: {:?}", numeros_ordenados);

// 17
println!("Desafio 17");
let texto = "Olá, mundo!";
let texto_reverso = reverse_string(texto);
println!("Texto reverso: {}", texto_reverso);

// 18
println!("Desafio 18");
let numeros = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
let numeros_pares = numeros_pares(numeros);
println!("Números pares: {:?}", numeros_pares);

//19
println!("Desafio 19");
let numeros = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
let numeros_impares = numeros_impares(numeros);
println!("Números ímpares: {:?}", numeros_impares);

//20
println!("Desafio 20");
let numeros = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
let soma = soma_lista(&numeros);
println!("Soma dos números : {}",soma);

//21
println!("Desafio 21");
let elementos = [1,2,3,4,5,6,7,8,9,0];
let quantos_numeros = conta_elementos(&elementos);
println!("Quantos números {}",quantos_numeros);

//22
println!("Desafio 22");
let mut numeros = vec![4.0, 2.0, 6.0, 1.0, 5.0, 3.0];
let mediana = calcular_mediana(&mut numeros);
println!("A mediana é: {}", mediana);

//23
println!("Desafio 22");
let numeros = vec![1, 2, 3, 4, 2, 3, 2, 1, 4, 4, 5];
let moda = calcular_moda(&numeros);
println!("A moda é: {:?}", moda);

//24
println!("Desafio 24");
let numeros = vec![2.0, 4.0, 6.0, 8.0, 10.0];
let desvio_padrao = desvio_padrao(&numeros);
println!("O desvio padrão é: {:.2}", desvio_padrao);

//25
println!("Desafio 25");
let numeros = vec![2.0, 4.0, 6.0, 8.0, 10.0];
let variancia = calcular_variancia(&numeros);
println!("A variância é: {:.2}", variancia);

//26
println!("Desafio 26");
let numeros = vec![1.0, 2.0, 3.0, 4.0, 5.0];
let cv = coeficiente_de_variacao(&numeros);
println!("Coeficiente de variação: {:?}", cv);

//27
println!("Desafio 27");
let lista_num: Vec<i32> = vec![1,2,4,3,6,5,7,9,8];
let mut lista_2 = lista_ao_quadrado(lista_num);
println!("Lista ao quadrado: {:?} ", lista_2);

//28
println!("Desafio 28");
let lista_num: Vec<i32> = vec![1,2,4,3,6,5,7,9,8];
let mut lista_3 = lista_ao_cubo(lista_num);
println!("Lista ao cubo: {:?} ", lista_3);

//29
println!("Desafio 29");
let lista_num: Vec<i32> = vec![1,2,4,3,6,5,7,9,8];
let potencia = 4;
let mut lista_pot = lista_na_potencia(lista_num,potencia);
println!("Lista na potência: {:?} ", lista_pot);

//30
println!("Desafio 30");
let lista_numeros: Vec<i32> = vec![1,-2,3,4,-5,-6,7];
let num_positivos = numeros_positivos(lista_numeros);
println!("Números positivos : {:?}",num_positivos);

//31
println!("Desafio 31");
let lista_numeros: Vec<i32> = vec![1,-2,3,4,-5,-6,7];
let num_negativos = numeros_negativos(lista_numeros);
println!("Números negativos : {:?}",num_negativos);

//32
println!("Desafio 32");
let lista_numeros: Vec<i32> = vec![1,-2,1,-3,3,4,-5,4,-5,-6,7];
let num_unicos = unique_numeros(lista_numeros);
println!("Números unicos : {:?}",num_unicos);

//33
println!("Desafio 33");
let lista1: Vec<i32> = vec![1,3,5,7,9];
let lista2: Vec<i32> = vec![1,5,9,2,3];
let num_comuns: Vec<i32> = elementos_comuns(lista1, lista2);
println!("Elementos comuns : {:?}",num_comuns);

//34
println!("Desafio 34");
let lista1: Vec<i32> = vec![1,3,5,7,9];
let lista2: Vec<i32> = vec![1,5,9,2,3];
let lista_mista: Vec<i32> = mistura_lista(lista1, lista2);
println!("Lista misturada : {:?}",lista_mista);

//35
println!("Desafio 35");
let lista_numeros : [i32;6] = [1,4,2,5,8,6];
let ordenado = estah_ordenado(&lista_numeros);
println!("Essa lista está ordenada ? : {}",ordenado);

//36
println!("Desafio 36");
let lista_numeros : [i32;8] = [8,7,6,5,4,3,2,1];
let ordenado = eh_ordem_decrescente(&lista_numeros);
println!("Essa lista está em ordem decrescente ? : {}",ordenado);

//37
println!("Desafio 37");
let lista_numeros : [i32;4] = [1,2,3,4];
let ordem = estah_ordenado(&lista_numeros);
println!("Essa lista está ordenada ? : {}",ordem);

//38
println!("Desafio 38");
let numeros = vec![2.37,3.12,4.53,5.27,6.32];
let lista_arredondada = arredonda_lista(&numeros);
println!("Lista arredondada {:?}",lista_arredondada);

//39
println!("Desafio 39");
let numeros = vec![10.2,20.5,30.75];
let formata_numeros = formata_moeda(&numeros);
println!("Moeda formatada ; {:?}",formata_numeros);

//40
println!("Desafio 40");
let mut vetor = vec!["Brasil".to_string(), "Zimbawe".to_string(), "Austrália".to_string()];
ordenar_strings(&mut vetor);
println!("{:?}", vetor);

//41
println!("Desafio 41");
let vetor = vec!["Brasil".to_string(), "Zimbawe".to_string(), "Austrália".to_string()];
let resultado = ordem_inversa_alfa(vetor);
println!("{:?}", resultado);

//42
println!("Desafio 42");
let vetor = vec![
        "Abacaxi".to_string(),
        "Pera".to_string(),
        "Banana".to_string()
    ];
let resultado = ordena_por_comprimento(vetor);
println!("{:?}", resultado);

//43
println!("Desafio 43");
let vetor = vec![
        "Abacaxi".to_string(),
        "Pera".to_string(),
        "Banana".to_string()
    ];
let resultado = strings_em_ordem_inversa(vetor);
println!("{:?}", resultado);

//44
println!("Desafio 44");
let vetor = vec![
        "Maçã".to_string(),
        "Banana".to_string(),
        "Abacaxi".to_string(),
    ];
let resultado = lista_em_maiusculas(vetor);
println!("{:?}", resultado);

//45
println!("Desafio 45");
let vetor = vec![
        "MAÇÃ".to_string(),
        "BANANA".to_string(),
        "ABACAXI".to_string(),
    ];
let resultado = para_minusculas(vetor);
println!("{:?}", resultado);

//46
println!("Desafio 46");
let texto = "Brasil!";
let resultado = substituir_vogais_com_numeros(texto);
println!("{}", resultado);

//47
println!("Desafio 47");
let palavra = "radar";
let resultado = eh_palindrome(palavra);
println!("A palavra '{}' é um palíndromo? {}", palavra, resultado);


//48
println!("Desafio 48");
let palavra1 = "amor";
let palavra2 = "roma";
let resultado = eh_anagrama(palavra1, palavra2);
println!("As palavras '{}' e '{}' são anagramas? {}", palavra1, palavra2, resultado);

//49
println!("Desafio 49");
let frase = "Eu gosto de programação";
let resultado = palavras_invertidas(frase);
println!("Frase original: {}", frase);
println!("Frase invertida: {}", resultado);

//50
println!("Desafio 50");
let texto = "O Brasil é o maior país da América do Sul!";
let resultado = remove_vogais(texto);
println!("Texto original: {}", texto);
println!("Texto sem vogais: {}", resultado);


}

//////////////////////////////////////////////////////////////////////////////////////

// CÓDIGO DE TODAS AS FUNÇÕES
// 01
fn imprime_nome(nome: &str) {
    println!("Meu nome completo é {}.",nome);
}

// 02
fn imprime_idade(idade: i32) {
    println!("Minha idade é {}", idade);
}

// 03
fn mostra_altura(altura: f32) {
    println!("Minha altura é {} metros.", altura);
}

// 04
fn imprime_peso(peso: f32) {
    println!("Meu peso é {} kg.", peso);
}

// 05
fn calcula_idade_em_meses(idade_anos: i32) {
    let idade_meses = idade_anos * 12;
    println!("Sua idade em meses é: {}.", idade_meses);
}

// 06
fn calcula_imc(peso: f32, altura: f32) -> f32 {
    let altura_quadrado = altura * altura;
    let imc = peso / (altura_quadrado);
    imc
}

// 07
fn imprime_numeros() {
    let numeros = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    for numero in numeros {
        println!("{}", numero);
    }
}

// 08
fn imprime_amigos() {
    let amigos = vec!["Alice", "José", "Carlos"];
    for amigo in amigos {
        println!("{}", amigo);
    }
}

// 09
fn imprime_frutas() {
    let frutas = ["maçã", "banana", "laranja", "kiwi", "pera"];
    for fruta in frutas.iter() {
        println!("{}", fruta);
    }
}

// 10 Dicionário
fn minhas_informacoes() {
    let informacoes = {
        let mut informacoes = std::collections::HashMap::new();
        informacoes.insert("nome", "João da Silva");
        informacoes.insert("idade", "27");
        informacoes.insert("altura", "1.75");
        informacoes.insert("peso", "68");
        informacoes
    };
    
    println!("Informações sobre mim:");
    for (chave, valor) in &informacoes {
        println!("{}: {}", chave, valor);
    }
}

// 11
fn soma_numeros(a: i32, b: i32) -> i32 {
    return a + b;
}

// 12
fn encontrar_maior_valor(numeros: &[i32]) -> Option<i32> {
    if numeros.is_empty() {
        return None;
    }
    let maior_valor = *numeros.iter().max().unwrap();
    Some(maior_valor)
}
    
// 13
fn encontrar_menor_valor(lista: &[i32]) -> Option<i32> {
    if lista.is_empty() {
        return None;
    }

    let menor_valor = *lista.iter().min().unwrap();
    Some(menor_valor)
}

// 14
fn calcula_media(lista: &[f64]) -> f64 {
    let soma: f64 = lista.iter().sum();
    let quantidade = lista.len() as f64;
    soma / quantidade
}

// 15
fn sort_list(lista: &mut Vec<i32>) -> Vec<i32> {
    lista.sort();
    lista.to_vec()
}

// 16
fn reverse_string(input: &str) -> String {
    input.chars().rev().collect()
}

// 17
fn numeros_pares(numeros: Vec<i32>) -> Vec<i32> {
    let mut numeros_pares = Vec::new();
    for numero in numeros {
        if numero % 2 == 0 {
            numeros_pares.push(numero);
        }
    }
    numeros_pares
}

// 18
fn numeros_impares(numeros: Vec<i32>) -> Vec<i32> {
    let mut numeros_impares = Vec::new();
    for numero in numeros {
        if numero % 2 != 0 {
            numeros_impares.push(numero);
        }
    }
    numeros_impares
}

// 19
fn soma_lista(numeros: &[i32]) -> i32 {
    let mut sum = 0;
    for num in numeros {
        sum += num;
    }
    sum
}

// 20
fn produto_da_lista(numeros: &[i32]) -> i32 {
    let mut product = 1;
    for num in numeros {
        product *= num;
    }
    product
}

// 21
fn conta_elementos(list: &[i32]) -> usize {
    list.len()
}

// 22
fn calcular_mediana(lista: &mut [f64]) -> f64 {
    lista.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let tamanho = lista.len();
    if tamanho % 2 == 0 {
        let meio = tamanho / 2;
        (lista[meio - 1] + lista[meio]) / 2.0
    } else {
        lista[tamanho / 2]
    }
}

// 23
fn calcular_moda(lista: &[i32]) -> Vec<i32> {
    let mut contador: HashMap<i32, usize> = HashMap::new();
    // Contar a frequência de cada número na lista
    for &numero in lista {
        let count = contador.entry(numero).or_insert(0);
        *count += 1;
    }
    // Encontrar a maior frequência
    let max_frequencia = contador.values().cloned().max().unwrap_or(0);
    // Filtrar os números que têm a maior frequência
    let moda: Vec<i32> = contador
        .into_iter()
        .filter(|(_, frequencia)| *frequencia == max_frequencia)
        .map(|(numero, _)| numero)
        .collect();
    moda
}

//24
fn desvio_padrao(lista: &[f64]) -> f64 {
    let tamanho = lista.len() as f64;
    // Calcula a média da lista
    let media: f64 = lista.iter().sum::<f64>() / tamanho;
    // Calcula a soma dos quadrados das diferenças em relação à média
    let soma_quadrados: f64 = lista.iter().map(|&x| (x - media).powi(2)).sum();
    // Calcula o desvio padrão
    let desvio_padrao = (soma_quadrados / tamanho).sqrt();
    desvio_padrao
}

// 25
fn calcular_variancia(lista: &[f64]) -> f64 {
    let tamanho = lista.len() as f64;
    // Calcula a média da lista
    let media: f64 = lista.iter().sum::<f64>() / tamanho;
    // Calcula a soma dos quadrados das diferenças em relação à média
    let soma_quadrados: f64 = lista.iter().map(|&x| (x - media).powi(2)).sum();
    // Calcula a variância
    let variancia = soma_quadrados / tamanho;
    variancia
}

// 26
fn coeficiente_de_variacao(numeros: &[f64]) -> Option<f64> {
    let n = numeros.len() as f64;
    if n <= 1.0 {
        return None;
    }
    let mean = numeros.iter().sum::<f64>() / n;
    let variancia = numeros.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / (n - 1.0);
    let desvio_padrao = variancia.sqrt();
    Some(100.0 * desvio_padrao / mean)
}

// 27
fn lista_ao_quadrado(numeros: Vec<i32>) -> Vec<i32> {
    let mut numeros_ao_quadrado = Vec::new();
    for numero in numeros {
        numeros_ao_quadrado.push(numero * numero);
    }
    numeros_ao_quadrado
}

// 28
fn lista_ao_cubo(numeros: Vec<i32>) -> Vec<i32> {
    let mut resultado = Vec::new();
    for num in numeros {
        resultado.push(num.pow(3));
    }
    resultado
}

// 29
fn lista_na_potencia(numeros: Vec<i32>, power: i32) -> Vec<i32> {
    let mut resultado = Vec::new();
    for num in numeros {
        resultado.push(num.pow(power as u32));
    }
    resultado
}

// 30
fn numeros_positivos(numeros: Vec<i32>) -> Vec<i32> {
    let mut resultado = Vec::new();
    for num in numeros {
        if num > 0 {
            resultado.push(num);
        }
    }
    resultado
}

// 31
fn numeros_negativos(numeros: Vec<i32>) -> Vec<i32> {
    let mut numeros_negativos = Vec::new();
    for numero in numeros {
        if numero < 0 {
            numeros_negativos.push(numero);
        }
    }
    numeros_negativos
}

// 32
fn numeros_diferentes(numeros: &[i32]) -> Vec<i32> {
    let mut numeros_diferentes = Vec::new();
    for &numero in numeros {
        if !numeros_diferentes.contains(&numero) {
            numeros_diferentes.push(numero);
        }
    }
    numeros_diferentes
}

// 33
fn unique_numeros(numeros: Vec<i32>) -> Vec<i32> {
    let mut unique_numeros = Vec::new();
    for numero in numeros {
        if !unique_numeros.contains(&numero) {
            unique_numeros.push(numero);
        }
    }
    unique_numeros
}

// 34
fn elementos_comuns(lista1: Vec<i32>, lista2: Vec<i32>) -> Vec<i32> {
    let mut resultado = Vec::new();
    for num in lista1 {
        if lista2.contains(&num) && !resultado.contains(&num) {
            resultado.push(num);
        }
    }
    resultado
}

// 35
fn mistura_lista(list1: Vec<i32>, list2: Vec<i32>) -> Vec<i32> {
    let mut lista_misturada = Vec::new();
    for num in list1 {
        if !lista_misturada.contains(&num) {
            lista_misturada.push(num);
        }
    }
    for num in list2 {
        if !lista_misturada.contains(&num) {
            lista_misturada.push(num);
        }
    }
    lista_misturada
}

// 36
fn estah_ordenado(numeros: &[i32]) -> bool {
    for i in 0..numeros.len()-1 {
        if numeros[i] > numeros[i+1] {
            return false;
        }
    }
    true
}

// 37
fn eh_ordem_decrescente(numeros: &[i32]) -> bool {
    for i in 1..numeros.len() {
        if numeros[i] > numeros[i-1] {
            return false;
        }
    }
    true
}

// 38
fn arredonda_lista(numeros: &[f64]) -> Vec<i32> {
    let mut arredonda_numeros = Vec::new();
    for numero in numeros {
        arredonda_numeros.push(numero.round() as i32);
    }
    arredonda_numeros
}

// 39
fn formata_moeda(numeros: &[f64]) -> Vec<String> {
    numeros.iter().map(|num| format!("R${:.2}", num)).collect()
}

// 40
fn ordenar_strings(vetor: &mut Vec<String>) {
    vetor.sort();
}

// 41
fn ordem_inversa_alfa(mut list: Vec<String>) -> Vec<String> {
    list.sort_by(|a, b| b.cmp(a));
    list
}

// 42
fn ordena_por_comprimento(mut strings: Vec<String>) -> Vec<String> {
    strings.sort_by(|a, b| a.len().cmp(&b.len()));
    strings
}

// 43
fn strings_em_ordem_inversa(list: Vec<String>) -> Vec<String> {
    let mut ordenada_list = list.clone();
    ordenada_list.sort_by(|a, b| b.len().cmp(&a.len()));
    ordenada_list
}

// 44
fn lista_em_maiusculas(list: Vec<String>) -> Vec<String> {
    let mut resultado = Vec::new();
    for s in list {
        resultado.push(s.to_uppercase());
    }
    resultado
}

// 45
fn para_minusculas(strings: Vec<String>) -> Vec<String> {
    let mut lowercase_strings = Vec::new();
    for string in strings {
        lowercase_strings.push(string.to_lowercase());
    }
    lowercase_strings
}

// 46
fn substituir_vogais_com_numeros(input: &str) -> String {
    let mut resultado = String::new();
    for c in input.chars() {
        match c {
            'a' => resultado.push('4'),
            'e' => resultado.push('3'),
            'i' => resultado.push('1'),
            'o' => resultado.push('0'),
            'u' => resultado.push('5'),
            _ => resultado.push(c),
        }
    }
    resultado
}

// 48
fn eh_palindrome(word: &str) -> bool {
    let reversed_word = word.chars().rev().collect::<String>();
    word == reversed_word
}

// 49
fn eh_anagrama(string1: &str, string2: &str) -> bool {
    let mut chars1: Vec<char> = string1.chars().collect();
    let mut chars2: Vec<char> = string2.chars().collect();
    chars1.sort();
    chars2.sort();
    chars1 == chars2
}

// 50
fn palavras_invertidas(sentence: &str) -> String {
    let mut palavras: Vec<&str> = sentence.split_whitespace().collect();
    palavras.reverse();
    palavras.join(" ")
}

// 51
fn remove_vogais(s: &str) -> String {
    let vowels = ['a', 'e','é', 'i','í', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
    let mut resultado = String::new();
    for c in s.chars() {
        if !vowels.contains(&c) {
            resultado.push(c);
        }
    }
    resultado
}