//! Simplistic Model Layer
//! (with mock-store layer)

use crate::{ctx::Ctx, Error, Result};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

// region:		--- Ticket Types

#[derive(Clone, Debug, Serialize)]
pub struct Ticket {
    pub id: u64,
    pub creator_user_id: u64,
    pub title: String,
}

#[derive(Deserialize)]
pub struct TicketToCreate {
    pub title: String,
}

// endregion:	--- Ticket Types

// region:		--- Model Controller

#[derive(Clone)]
pub struct ModelController {
    tickets_store: Arc<Mutex<Vec<Option<Ticket>>>>,
}

// Constructor

impl ModelController {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            tickets_store: Arc::default(),
        })
    }
}

// CRUD Implementations

impl ModelController {
    pub async fn create_ticket(
        &self,
        context: Ctx,
        ticket_to_create: TicketToCreate,
    ) -> Result<Ticket> {
        let mut store = self.tickets_store.lock().unwrap();

        let id = store.len() as u64;

        let ticket = Ticket {
            id,
            creator_user_id: context.user_id(),
            title: ticket_to_create.title,
        };

        store.push(Some(ticket.clone()));

        Ok(ticket)
    }

    pub async fn list_tickets(&self, _context: Ctx) -> Result<Vec<Ticket>> {
        let store = self.tickets_store.lock().unwrap();

        let tickets = store.iter().filter_map(|t| t.clone()).collect();

        Ok(tickets)
    }

    pub async fn delete_ticket(&self, _context: Ctx, id: u64) -> Result<Ticket> {
        let mut store = self.tickets_store.lock().unwrap();

        let ticket = store.get_mut(id as usize).and_then(|t| t.take());

        ticket.ok_or(Error::TicketDeleteFailIdNotFound { id })
    }
}

// endregion:	--- Model Controller
