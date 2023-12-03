use crate::{
    ctx::Ctx,
    model::{ModelController, Ticket, TicketToCreate},
    Result,
};
use axum::{
    extract::{Path, State},
    routing::{delete, post},
    Json, Router,
};

pub fn routes(model_controller: ModelController) -> Router {
    Router::new()
        .route("/tickets", post(create_ticket).get(list_tickets))
        .route("/tickets/:id", delete(delete_ticket))
        .with_state(model_controller)
}

// region:		--- REST Handler

async fn create_ticket(
    model_controller: State<ModelController>,
    context: Ctx,
    Json(ticket_to_create): Json<TicketToCreate>,
) -> Result<Json<Ticket>> {
    println!("-->> {:<20} - create_ticket", "HANDLER");

    let ticket = model_controller
        .create_ticket(context, ticket_to_create)
        .await?;

    Ok(Json(ticket))
}

async fn list_tickets(
    State(model_controller): State<ModelController>,
    context: Ctx,
) -> Result<Json<Vec<Ticket>>> {
    println!("-->> {:<20} - list_tickets", "HANDLER");

    let tickets = model_controller.list_tickets(context).await?;

    Ok(Json(tickets))
}

async fn delete_ticket(
    State(model_controller): State<ModelController>,
    context: Ctx,
    Path(id): Path<u64>,
) -> Result<Json<Ticket>> {
    println!("-->> {:<20} - delete_ticket", "HANDLER");

    let ticket = model_controller.delete_ticket(context, id).await?;

    Ok(Json(ticket))
}

// endregion:	--- REST Handler
