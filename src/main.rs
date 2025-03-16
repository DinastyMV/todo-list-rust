mod helpers;
mod todo;

use crate::helpers::*;
use crate::todo::TodoList;
use std::io;

fn main() {
    let mut todo_list = match TodoList::load("todolist.json") {
        Ok(list) => list,
        Err(_) => TodoList::new(),
    };

    loop {
        println!("\nEscolha uma ação: ");
        println!("1 - Adicionar tarefa");
        println!("2 - Concluir tarefa");
        println!("3 - Remover tarefa");
        println!("4 - Exibir tarefas");
        println!("5 - Editar tarefa");
        println!("6 - Sair");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Falhar ao escolher ação!");
        let choice: u32 = choice.trim().parse().expect("Entrada inválida!");

        match choice {
            1 => add_task_input(&mut todo_list),
            2 => complete_task_input(&mut todo_list),
            3 => remove_task_input(&mut todo_list),
            4 => show_tasks_input(&mut todo_list),
            5 => edit_task_input(&mut todo_list),
            6 => {
                if let Err(e) = todo_list.save("todolist.json") {
                    eprintln!("Erro ao salvar a lista de tarefas: {}", e);
                } else {
                    println!("Lista de tarefas salva com sucesso!");
                }
                println!("Saindo...");
                break;
            }
            _ => {
                println!("Valor inválido!");
            }
        } // Match
    } // Loop
}


