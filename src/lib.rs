use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};
use yew::services::ConsoleService;


pub struct MyApp {
    console: ConsoleService,
    
    persons: Vec<Person>,
}

pub struct Person {
    id: i32,
    account: String,
    name: String,
    age: i32,
}

pub enum Msg {
    Click,
}

impl Component for MyApp {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        let mut persons: Vec<Person> = vec![];
        persons.push(Person {
            id: 1,
            account: "tanggang".to_string(),
            name: "唐刚".to_string(),
            age: 31
        });
        persons.push(Person {
            id: 2,
            account: "manning".to_string(),
            name: "曼宁".to_string(),
            age: 29
        });
        persons.push(Person {
            id: 3,
            account: "xusong".to_string(),
            name: "许崇".to_string(),
            age: 20
        });
        
        MyApp {
            console: ConsoleService::new(),
            persons
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Click => {
                self.console.log("clicked")
            }
        }
        true
    }
}

impl Renderable<MyApp> for MyApp {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <table>
                    <thead>
                        <th>
                            <td>{"id"}</td>
                            <td>{"姓名"}</td>
                            <td>{"账号"}</td>
                            <td>{"年龄"}</td>
                        </th>
                    </thead>
                    <tbody>
                        { for self.persons.iter().map(|person|
                            row_view(person)
                        )}
                    </tbody>
                </table>
                
                <button onclick=|_| Msg::Click,>{ "Click" }</button>
            </div>
        }
    }
}

fn row_view(person: &Person) -> Html<MyApp> {
    html! {
        <tr>
            <td>{ person.id }</td>
            <td>{ person.account.clone() }</td>
            <td>{ person.name.clone() }</td>
            <td>{ person.age }</td>
        </tr>
    }
}

/*
fn view_square(selected: Option<(u32, u32)>, row: u32, column: u32) -> Html<Model> {
    html! {
        <td
            class=square_class((column, row), selected),
            onclick=|_| Msg::Select(column, row),
        >
        </td>
    }
}

fn view_row(selected: Option<(u32, u32)>, row: u32) -> Html<Model> {
    html! {
        <tr>
            {for (0..99).map(|column| {
                view_square(selected, row, column)
            })}
        </tr>
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <table>
                {for (0..99).map(|row| {
                    view_row(self.selected, row)
                })}
            </table>
        }
    }
}
*/
