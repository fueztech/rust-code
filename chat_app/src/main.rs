#[macro_use] extern crate rocket;

use rocket::{tokio::sync::broadcast::{channel, Sender, error::RecvError}, serde::{Deserialize, Serialize}, State, Shutdown, response::stream::{EventStream, Event}, fs::{relative, FileServer}};
use rocket::form::Form;
use rocket::tokio::select;

#[derive(Debug, Clone, FromForm, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]

struct Message {
    #[field(validate = len(..30))] //room can only be 30 characters long
    pub room: String, // Chat app will have rooms, users, and messages.
    #[field(validate = len(..20))] // user & message can only be 20 characters long
    pub username: String,
    pub message: String,

}

#[post("/message", data = "<form>")]
fn post(form: Form<Message>, queue: &State<Sender<Message>>) {
    // A send 'fails' if there are no active subscribers. Thats fine.
    let _res = queue.send(form.into_inner());
}

#[get("/events")]
async fn events(queue: &State<Sender<Message>>, mut end: Shutdown) -> EventStream![] {
    let mut rx = queue.subscribe();

        EventStream! {
            loop {
                let msg = select! {
                    msg = rx.recv() => match msg {
                        Ok(msg) => msg,
                        Err(RecvError::Closed) => break,
                        Err(RecvError::Lagged(_)) => continue,
                    },
                    _ = &mut end => break,
                };

                yield Event::json(&msg);
            }
        }
}
#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage( channel::<Message>(1024).0)
        .mount("/", routes![post, events])
        .mount("/", FileServer::from(relative!("static")))
}