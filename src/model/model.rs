use crate::{
    ctx::Ctx,
    error::{Error, Result},
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Serialize)]
pub struct Ticket {
    pub id: u64,
    pub cid: u64, // creator user_id
    pub title: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TicketForCreate {
    pub title: String,
}

// Memory Store
#[derive(Clone)]
pub struct ModelController {
    tickets_store: Arc<Mutex<Vec<Option<Ticket>>>>,
}

impl ModelController {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            tickets_store: Arc::default(),
        })
    }
}

// CRUD Implementation
impl ModelController {
    pub async fn create_ticket(&self, ctx: Ctx, ticket_fc: TicketForCreate) -> Result<Ticket> {
        let mut store = self.tickets_store.lock().unwrap();

        let id = store.len() as u64;

        let ticket = Ticket {
            id,
            cid: ctx.user_id(),
            title: ticket_fc.title,
        };

        store.push(Some(ticket.clone()));

        Ok(ticket)
    }

    pub async fn list_tickets(&self, _ctx: Ctx) -> Result<Vec<Ticket>> {
        let store = self.tickets_store.lock().unwrap();

        let tickets = store.iter().filter_map(|ticket| ticket.clone()).collect();

        Ok(tickets)
    }

    pub async fn delete_ticket(&self, _ctx: Ctx, id: u64) -> Result<Ticket> {
        let mut store = self.tickets_store.lock().unwrap();
        let ticket = store.get_mut(id as usize).and_then(|ticket| ticket.take());

        ticket.ok_or(Error::TicketDeleteFailedNotFound { id })
    }
}
