/// Importamos as bibliotecas necessárias:
///
/// - `chrono` para manipulação de datas e tempos.
/// - `serde` para serialização e deserialização de JSON.
/// - `std::fs` e `std::io` para manipulação de arquivos.
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io;

/// Representa o estado de uma tarefa, que pode estar pendente ou concluída.
#[derive(Debug, Serialize, Deserialize)]
pub enum TaskStatus {
    Pending,
    Completed,
}

/// Representa a prioridade de uma tarefa. Pode ser alta, média, baixa, ou sem prioridade.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum TaskPreference {
    Alto,
    Medio,
    Baixo,
    Vazio,
}

/// Define a estrutura de uma tarefa, contendo:
/// - `title`: O título da tarefa.
/// - `status`: O estado da tarefa (pendente ou concluída).
/// - `preference`: A prioridade da tarefa.
/// - `created_at`: A data de criação da tarefa.
/// - `completed_at`: A data de conclusão (opcional).
/// - `description`: A descrição da tarefa.
/// - `completed`: Indica se a tarefa foi concluída.
#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub title: String,
    pub status: TaskStatus,
    pub preference: TaskPreference,
    pub created_at: DateTime<Utc>,           // Data de criação
    pub completed_at: Option<DateTime<Utc>>, // Data de conclusão, opcional
    pub description: String,
    pub completed: bool,
}

/// Implementação de métodos para a struct `Task`.
impl Task {
    /// Cria uma nova tarefa com título e descrição fornecidos.
    /// Define o status como `Pending` e a prioridade como `Vazio`.
    pub fn new(title: String, description: String) -> Task {
        Task {
            title,
            status: TaskStatus::Pending,
            preference: TaskPreference::Vazio,
            created_at: Utc::now(),
            completed_at: None,
            description,
            completed: false,
        }
    }

    /// Marca a tarefa como concluída, atualizando o status e a data de conclusão.
    pub fn complete(&mut self) {
        self.status = TaskStatus::Completed;
        self.completed_at = Some(Utc::now());
    }

    /// Exibe o status atual da tarefa, incluindo a data de criação e conclusão, se aplicável.
    pub fn show_status(&self) {
        match self.status {
            TaskStatus::Pending => println!("Tarefa pendente: {}", self.title),
            TaskStatus::Completed => {
                println!("Tarefa concluída: {}", self.title);
                if let Some(completed_at) = self.completed_at {
                    println!("Concluída em: {}", completed_at.format("%Y-%m-%d %H:%M:%S"));
                }
            }
        }
        println!("Criada em: {}", self.created_at.format("%Y-%m-%d %H:%M:%S"));
    }

    /// Define a prioridade da tarefa como alta.
    pub fn high(&mut self) {
        self.preference = TaskPreference::Alto;
    }

    /// Define a prioridade da tarefa como média.
    pub fn average(&mut self) {
        self.preference = TaskPreference::Medio;
    }

    /// Define a prioridade da tarefa como baixa.
    pub fn low(&mut self) {
        self.preference = TaskPreference::Baixo;
    }

    /// Exibe a prioridade da tarefa.
    pub fn show_preference(&self) {
        match self.preference {
            TaskPreference::Alto => println!("Tarefa com alta prioridade: {}", self.title),
            TaskPreference::Medio => println!("Tarefa com media prioridade: {}", self.title),
            TaskPreference::Baixo => println!("Tarefa com baixa prioridade: {}", self.title),
            TaskPreference::Vazio => println!("Tarefa sem prioridade: {}", self.title),
        }
    }
}

/// Representa uma lista de tarefas.
#[derive(Serialize, Deserialize)]
pub struct TodoList {
    pub tasks: Vec<Task>,
}

/// Implementação de métodos para a lista de tarefas `TodoList`.
impl TodoList {
    /// Cria uma nova lista de tarefas vazia.
    pub fn new() -> TodoList {
        TodoList { tasks: Vec::new() }
    }

    /// Salva a lista de tarefas em um arquivo JSON.
    pub fn save(&self, filename: &str) -> io::Result<()> {
        let file = File::create(filename)?;
        serde_json::to_writer(file, self)?;
        Ok(())
    }

    /// Carrega uma lista de tarefas de um arquivo JSON.
    pub fn load(filename: &str) -> io::Result<Self> {
        let file = File::open(filename)?;
        let todo_list = serde_json::from_reader(file)?;
        Ok(todo_list)
    }

    /// Adiciona uma nova tarefa à lista.
    pub fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }

    /// Remove uma tarefa da lista pelo índice.
    pub fn remove_task(&mut self, index: usize) {
        if index < self.tasks.len() {
            self.tasks.remove(index);
        } else {
            println!("Índice inválido!");
        }
    }

    /// Marca uma tarefa como concluída pelo índice.
    pub fn complete_task(&mut self, index: usize) {
        if let Some(task) = self.tasks.get_mut(index) {
            task.complete();
        } else {
            println!("Índice inválido!");
        }
    }

    /// Exibe todas as tarefas, incluindo informações como título, status, prioridade e datas.
    pub fn show_tasks(&self) {
        for (i, task) in self.tasks.iter().enumerate() {
            println!("{}:", i);
            println!("  Título: {}", task.title);
            println!("  Status: {:?}", task.status);
            println!("  Prioridade: {:?}", task.preference);
            println!(
                "  Criada em: {}",
                task.created_at.format("%Y-%m-%d %H:%M:%S")
            );

            if let Some(completed_at) = task.completed_at {
                println!(
                    "  Concluída em: {}",
                    completed_at.format("%Y-%m-%d %H:%M:%S")
                );
            }

            println!(" ");
        }
    }

    /// Exibe apenas as tarefas concluídas.
    pub fn show_tasks_completed(&self) {
        for (i, task) in self.tasks.iter().enumerate() {
            if let Some(TaskStatus::Completed) = self.get_task_status(i) {
                println!("{}:", i);
                println!("  Título: {}", task.title);
                println!("  Status: {:?}", task.status);
                println!("  Prioridade: {:?}", task.preference);
                println!(
                    "  Criada em: {}",
                    task.created_at.format("%Y-%m-%d %H:%M:%S")
                );

                if let Some(completed_at) = task.completed_at {
                    println!(
                        "  Concluída em: {}",
                        completed_at.format("%Y-%m-%d %H:%M:%S")
                    );
                }

                println!(" ");
            }
        }
    }

    /// Exibe apenas as tarefas pendentes.
    pub fn show_tasks_pending(&self) {
        for (i, task) in self.tasks.iter().enumerate() {
            if let Some(TaskStatus::Pending) = self.get_task_status(i) {
                println!("{}:", i);
                println!("  Título: {}", task.title);
                println!("  Status: {:?}", task.status);
                println!("  Prioridade: {:?}", task.preference);
                println!(
                    "  Criada em: {}",
                    task.created_at.format("%Y-%m-%d %H:%M:%S")
                );

                println!(" ");
            }
        }
    }

    /// Retorna o status de uma tarefa pelo índice.
    pub fn get_task_status(&self, index: usize) -> Option<&TaskStatus> {
        self.tasks.get(index).map(|task| &task.status)
    }

    /// Edita o título de uma tarefa pelo índice.
    pub fn edit_task_title(&mut self, index: usize, new_title: String) {
        if let Some(task) = self.tasks.get_mut(index) {
            task.title = new_title.trim().to_string();
        } else {
            println!("Tarefa não encontrada!");
        }
    }

    /// Edita a prioridade de uma tarefa pelo índice.
    pub fn edit_task_preference(&mut self, index: usize, new_preference: TaskPreference) {
        if let Some(task) = self.tasks.get_mut(index) {
            task.preference = new_preference;
        } else {
            println!("Tarefa não encontrada!");
        }
    }

    /// Retorna a prioridade de uma tarefa pelo índice.
    pub fn get_task_preference(&self, index: usize) -> Option<&TaskPreference> {
        self.tasks.get(index).map(|task| &task.preference)
    }

    /// Exibe tarefas pendentes sem prioridade definida.
    pub fn show_tasks_preference_void(&self) {
        for (i, task) in self.tasks.iter().enumerate() {
            if let (Some(TaskPreference::Vazio), Some(TaskStatus::Pending)) =
                (self.get_task_preference(i), self.get_task_status(i))
            {
                println!("{}:", i);
                println!("  Título: {}", task.title);
                println!("  Status: {:?}", task.status);
                println!("  Prioridade: {:?}", task.preference);
                println!(
                    "  Criada em: {}",
                    task.created_at.format("%Y-%m-%d %H:%M:%S")
                );

                println!(" ");
            }
        }
    }
    /// Exibe as tarefas pendentes com prioridade definida
    pub fn show_tasks_preference_invoid(&self) {
        for (i, task) in self.tasks.iter().enumerate() {
            if let (Some(preference), Some(TaskStatus::Pending)) =
                (self.get_task_preference(i), self.get_task_status(i))
            {
                if *preference != TaskPreference::Vazio {
                    println!("{}:", i);
                    println!("  Título: {}", task.title);
                    println!("  Status: {:?}", task.status);
                    println!("  Prioridade: {:?}", task.preference);
                    println!(
                        "  Criada em: {}",
                        task.created_at.format("%Y-%m-%d %H:%M:%S")
                    );

                    if let Some(completed_at) = task.completed_at {
                        println!(
                            "  Concluída em: {}",
                            completed_at.format("%Y-%m-%d %H:%M:%S")
                        );
                    }

                    println!(" ");
                }
            }
        }
    }
}
