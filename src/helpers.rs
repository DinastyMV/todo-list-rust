use crate::todo::Task;
use crate::todo::TaskPreference;
use crate::todo::TodoList;
use std::io;

/// Função que adiciona uma nova tarefa à lista de tarefas.
///
/// Ela solicita ao usuário que insira o título e a descrição da tarefa.
/// Uma nova tarefa é criada com o status de "pendente" e adicionada à lista.
///
/// # Parâmetros
/// - `todo_list`: A lista de tarefas (`TodoList`) onde a nova tarefa será adicionada.
pub fn add_task_input(todo_list: &mut TodoList) {
    println!("Digite o título e a descrição da sua tarefa:");

    let mut title = String::new();
    io::stdin()
        .read_line(&mut title)
        .expect("Falha ao ler título!");

    let mut description = String::new();
    io::stdin()
        .read_line(&mut description)
        .expect("Falha ao ler descrição!");

    let task = Task::new(title.trim().to_string(), description.trim().to_string());

    todo_list.add_task(task); // Adiciona a tarefa à lista
}

/// Função para marcar uma tarefa como concluída.
///
/// Ela exibe todas as tarefas atuais e solicita que o usuário selecione o índice da tarefa a ser concluída.
///
/// # Parâmetros
/// - `todo_list`: A lista de tarefas (`TodoList`) da qual uma tarefa será marcada como concluída.
pub fn complete_task_input(todo_list: &mut TodoList) {
    todo_list.show_tasks();

    println!("Digite o número da tarefa que deseja concluir:");

    let mut index = String::new();
    io::stdin().read_line(&mut index).expect("Valor inválido!");

    let indice: usize = index.trim().parse().expect("Falha ao ler valor!");

    todo_list.complete_task(indice); // Marca a tarefa como concluída
}

/// Função para remover uma tarefa da lista.
///
/// Ela exibe todas as tarefas atuais e solicita que o usuário selecione o índice da tarefa a ser removida.
///
/// # Parâmetros
/// - `todo_list`: A lista de tarefas (`TodoList`) da qual uma tarefa será removida.
pub fn remove_task_input(todo_list: &mut TodoList) {
    todo_list.show_tasks();

    println!("Digite o número da tarefa que deseja remover:");

    let mut index = String::new();
    io::stdin().read_line(&mut index).expect("Valor inválido!");

    let indice: usize = index.trim().parse().expect("Falha ao ler valor!");

    todo_list.remove_task(indice); // Remove a tarefa selecionada
}

/// Função para exibir tarefas com base no status selecionado pelo usuário.
///
/// O usuário escolhe entre visualizar todas as tarefas, apenas as concluídas ou as pendentes.
///
/// # Parâmetros
/// - `todo_list`: A lista de tarefas (`TodoList`) que será exibida com base na seleção do usuário.
pub fn show_tasks_input(todo_list: &mut TodoList) {
    println!("Quais tarefas deseja ver?");
    println!("1 - Todas");
    println!("2 - Completas");
    println!("3 - Pendentes");

    let mut index = String::new();
    io::stdin().read_line(&mut index).expect("Valor inválido!");
    let input: usize = index.trim().parse().expect("Falha ao ler valor!");

    match input {
        1 => todo_list.show_tasks(),           // Mostra todas as tarefas
        2 => todo_list.show_tasks_completed(), // Mostra apenas tarefas concluídas
        3 => todo_list.show_tasks_pending(),   // Mostra apenas tarefas pendentes
        _ => println!("Índice inválido!"),
    }
}

/// Função para editar o título ou prioridade de uma tarefa.
///
/// O usuário pode escolher alterar o título, adicionar ou mudar a prioridade de uma tarefa.
///
/// # Parâmetros
/// - `todo_list`: A lista de tarefas (`TodoList`) cujas tarefas poderão ser editadas.
pub fn edit_task_input(todo_list: &mut TodoList) {
    println!("Qual edição deseja realizar?");
    println!("1 - Mudar o título");
    println!("2 - Adicionar uma prioridade");
    println!("3 - Mudar a prioridade");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Valor inválido!");
    let input: usize = input.trim().parse().expect("Falha ao ler valor!");

    match input {
        1 => {
            todo_list.show_tasks_pending(); // Mostra as tarefas pendentes

            println!("Qual tarefa deseja mudar o título?");

            let mut task_input = String::new();
            io::stdin()
                .read_line(&mut task_input)
                .expect("Valor inválido!");
            let index: usize = task_input.trim().parse().expect("Falha ao ler valor!");

            println!("Qual vai ser o novo título?");
            let mut new_title = String::new();
            io::stdin()
                .read_line(&mut new_title)
                .expect("Falha ao ler String!");

            todo_list.edit_task_title(index, new_title.trim().to_string()); // Atualiza o título da tarefa
        }
        2 => {
            todo_list.show_tasks_preference_void(); // Mostra tarefas pendentes sem prioridade

            println!("Qual tarefa deseja adicionar uma prioridade?");
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Valor inválido!");
            let index: usize = input.trim().parse().expect("Falha ao ler valor!");

            println!("Qual vai ser a prioridade?");
            println!("1 - Alto");
            println!("2 - Médio");
            println!("3 - Baixo");

            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Valor inválido!");
            let prioridade: usize = input.trim().parse().expect("Falha ao ler valor!");

            let new_preference = match prioridade {
                1 => TaskPreference::Alto,
                2 => TaskPreference::Medio,
                3 => TaskPreference::Baixo,
                _ => TaskPreference::Vazio,
            };

            todo_list.edit_task_preference(index, new_preference); // Atualiza a prioridade da tarefa
        }
        3 => {
            todo_list.show_tasks_preference_invoid(); // Mostra tarefas com prioridade definida

            println!("Qual tarefa deseja mudar a prioridade?");
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Valor inválido!");
            let index: usize = input.trim().parse().expect("Falha ao ler valor!");

            println!("Qual vai ser a nova prioridade?");
            println!("1 - Alto");
            println!("2 - Médio");
            println!("3 - Baixo");

            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Valor inválido!");
            let prioridade: usize = input.trim().parse().expect("Falha ao ler valor!");

            let new_preference = match prioridade {
                1 => TaskPreference::Alto,
                2 => TaskPreference::Medio,
                3 => TaskPreference::Baixo,
                _ => TaskPreference::Vazio,
            };

            todo_list.edit_task_preference(index, new_preference); // Atualiza a prioridade da tarefa
        }
        _ => println!("Valor inválido!"),
    }
}
