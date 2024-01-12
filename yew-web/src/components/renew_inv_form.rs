use std::collections::HashMap;

use chrono::{DateTime, NaiveDate, TimeZone, Utc};
use web_sys::wasm_bindgen::JsCast;
use web_sys::{HtmlSelectElement, MouseEvent};
use yew::events::{Event, InputEvent};
use yew::{html, Callback, Component, Html, Properties};

use super::base_inv_form::BaseFormComponent;
use types::Investment;

#[derive(Properties, PartialEq, Clone)]
pub struct RenewInvForm {
    form_changed: bool,
    show_renew_confirmation: bool,
    props: RenewInvFormProps,
    base: BaseFormComponent,
}

#[derive(Properties, PartialEq, Clone)]
pub struct RenewInvFormProps {
    pub edit_investment: Callback<Investment>,
    pub investment: Investment,
    pub on_renew: Callback<()>,
}

pub enum Msg {
    ValidateAndSave(String, String),
    ValidateDateAndSave(String, Option<DateTime<Utc>>),
    ConfirmRenewForm,
    CancelRenewForm,
    RenewForm,
}

impl Component for RenewInvForm {
    type Message = Msg;
    type Properties = RenewInvFormProps;

    fn create(ctx: &yew::Context<Self>) -> Self {
        Self {
            form_changed: false,
            show_renew_confirmation: false,
            props: RenewInvFormProps {
                edit_investment: ctx.props().edit_investment.clone(),
                investment: ctx.props().investment.clone(),
                on_renew: ctx.props().on_renew.clone(),
            },
            base: BaseFormComponent {
                error_messages: HashMap::new(),
            },
        }
    }

    fn update(&mut self, _ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ValidateAndSave(field, value) => {
                match field.as_str() {
                    "inv-name" => {
                        self.props.investment.inv_name = value;
                    }
                    "name" => {
                        self.props.investment.name = value;
                    }
                    "inv-type" => {
                        self.props.investment.inv_type = value;
                    }
                    "return-type" => {
                        self.props.investment.return_type = value;
                    }
                    "inv-amount" => {
                        self.props.investment.inv_amount = value.parse().unwrap_or(0);
                    }
                    "return-amount" => {
                        self.props.investment.return_amount = value.parse().unwrap_or(0);
                    }
                    "return-rate" => {
                        self.props.investment.return_rate = value.parse().unwrap_or(0);
                    }
                    _ => {}
                }
                self.base.error_messages.remove(field.as_str());
                self.form_changed = true;
            }
            Msg::ValidateDateAndSave(field, date) => {
                match field.as_str() {
                    "start-date" => {
                        self.props.investment.start_date = date;
                    }
                    "end-date" => {
                        self.props.investment.end_date = date;
                    }
                    _ => {}
                }
                self.base.error_messages.remove(field.as_str());
                self.form_changed = true;
            }
            Msg::ConfirmRenewForm => {
                if self.save_form() {
                    self.props.on_renew.emit(());
                }
            }
            Msg::CancelRenewForm => {
                self.show_renew_confirmation = false;
            }
            Msg::RenewForm => {
                self.show_renew_confirmation = true;
            }
        }
        true
    }

    fn view(&self, ctx: &yew::Context<Self>) -> Html {
        html! {
            <div class="mx-auto w-full relative">
                <form>
                    <div class="grid gap-6 mb-6 md:grid-cols-2 lg:grid-cols-3 text-text-950">
                        { self.date_field(ctx, "start-date", &self.props.investment.start_date.map(|d| d.format("%Y-%m-%d").to_string()).unwrap_or_default()) }
                        { self.date_field(ctx, "end-date", &self.props.investment.end_date.map(|d| d.format("%Y-%m-%d").to_string()).unwrap_or_default()) }
                        { self.input_field(ctx, "inv-name", "text", &self.props.investment.inv_name) }
                        { self.input_field(ctx, "name", "text", &self.props.investment.name) }
                        { self.select_field(ctx, "inv-type", &self.props.investment.inv_type,
                            html! {
                                <>
                                    <option value="FD" selected={self.props.investment.inv_type == "FD"}>{"FD"}</option>
                                    <option value="RD" selected={self.props.investment.inv_type == "RD"}>{"RD"}</option>
                                </>
                            }
                        ) }
                        { self.select_field(ctx, "return-type", &self.props.investment.return_type,
                            html! {
                                <>
                                    <option value="Ordinary" selected={self.props.investment.return_type == "Ordinary"}>{"Ordinary"}</option>
                                    <option value="Culmulative" selected={self.props.investment.return_type == "Culmulative"}>{"Culmulative"} </option>
                                </>
                            }
                        ) }
                        { self.input_field(ctx, "return-amount", "number", &self.props.investment.return_amount.to_string()) }
                        { self.input_field(ctx, "inv-amount", "number", &self.props.investment.inv_amount.to_string()) }
                        { self.input_field(ctx, "return-rate", "number", &self.props.investment.return_rate.to_string()) }
                        <button type="submit" disabled={!self.form_changed}
                            onclick={ctx.link().callback(|e: MouseEvent| {
                                // prevent the webpage from moving to top when the button is clicked
                                e.prevent_default();
                                Msg::RenewForm
                            })}
                            class={format!("{} {}", {if self.form_changed { "bg-primary-600 hover:bg-primary-700" } else { "bg-background-500" }}, "inline-flex justify-center items-center px-5 py-2.5 mt-3 sm:mt-5 text-sm font-medium text-center text-text-50 rounded-lg focus:ring-4 focus:ring-primary-200")}>
                            {"Renew"}
                        </button>
                    </div>
                </form>
                {if self.show_renew_confirmation {
                    html! {
                        <div class="absolute inset-0 flex items-center justify-center bg-white dark:bg-black bg-opacity-80 dark:bg-opacity-70">
                            <div class="bg-background-50 p-4 rounded text-text-950">
                                <p class="mb-2">{"Are you sure you want to renew this Investment?"}</p>
                                <div class="flex justify-center">
                                    <button onclick={ctx.link().callback(|_| Msg::ConfirmRenewForm)} class="bg-red-500 px-4 py-2 mr-1 rounded">{"Confirm"}</button>
                                    <button onclick={ctx.link().callback(|_| Msg::CancelRenewForm)} class="bg-background-500 px-4 py-2 ml-1 rounded">{"Cancel"}</button>
                                </div>
                            </div>
                        </div>
                    }
                } else { html! {} } }
            </div>
        }
    }
}

impl RenewInvForm {
    fn input_field(
        &self,
        ctx: &yew::Context<Self>,
        field_id: &str,
        field_type: &str,
        field_value: &str,
    ) -> Html {
        let field_id_str = field_id.to_string();
        let on_input = ctx.link().callback(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target().unwrap().dyn_into().unwrap();
            Msg::ValidateAndSave(field_id_str.clone(), input.value())
        });
        self.base
            .input_field(field_id, field_type, field_value, on_input)
    }

    fn select_field(
        &self,
        ctx: &yew::Context<Self>,
        field_id: &str,
        field_value: &str,
        options: Html,
    ) -> Html {
        let field_id_str = field_id.to_string();
        let on_change = ctx.link().callback(move |e: Event| {
            let target = e.target().unwrap();
            let select_element = target.dyn_into::<HtmlSelectElement>().unwrap();
            let value = select_element.value();
            Msg::ValidateAndSave(field_id_str.clone(), value)
        });
        self.base
            .select_field(field_id, field_value, options, on_change)
    }

    fn date_field(&self, ctx: &yew::Context<Self>, field_id: &str, field_value: &str) -> Html {
        let field_id_str = field_id.to_string();
        let on_input = ctx.link().callback(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target().unwrap().dyn_into().unwrap();
            let date = NaiveDate::parse_from_str(&input.value(), "%Y-%m-%d")
                .map(|date| {
                    date.and_hms_opt(0, 0, 0)
                        .map(|datetime| Utc.from_utc_datetime(&datetime))
                })
                .ok()
                .flatten();
            Msg::ValidateDateAndSave(field_id_str.clone(), date)
        });

        self.base.date_field(field_id, field_value, on_input)
    }

    fn validate_form(&mut self) -> bool {
        let mut is_valid = true;

        if self.props.investment.inv_name.is_empty() {
            self.base.error_messages.insert(
                "inv-name".to_string(),
                "Investment Name can not be blank".to_string(),
            );
            is_valid = false;
        }

        if self.props.investment.name.is_empty() {
            self.base
                .error_messages
                .insert("name".to_string(), "Name can not be blank".to_string());
            is_valid = false;
        }

        if self.props.investment.inv_type.is_empty() {
            self.base.error_messages.insert(
                "inv-type".to_string(),
                "Investment Type can not be blank".to_string(),
            );
            is_valid = false;
        }

        if self.props.investment.return_type.is_empty() {
            self.base.error_messages.insert(
                "return-type".to_string(),
                "Return Type can not be blank".to_string(),
            );
            is_valid = false;
        }

        if self.props.investment.inv_amount == 0 {
            self.base.error_messages.insert(
                "inv-amount".to_string(),
                "Investment Amount can not be blank".to_string(),
            );
            is_valid = false;
        }

        if self.props.investment.return_amount == 0 {
            self.base.error_messages.insert(
                "return-amount".to_string(),
                "Return Amount can not be blank".to_string(),
            );
            is_valid = false;
        }

        if self.props.investment.return_rate == 0 {
            self.base.error_messages.insert(
                "return-rate".to_string(),
                "Return Rate can not be blank".to_string(),
            );
            is_valid = false;
        }

        if self.props.investment.start_date.is_none() {
            self.base.error_messages.insert(
                "start-date".to_string(),
                "Start Date can not be blank".to_string(),
            );
            is_valid = false;
        }

        if self.props.investment.end_date.is_none() {
            self.base.error_messages.insert(
                "end-date".to_string(),
                "End Date can not be blank".to_string(),
            );
            is_valid = false;
        }

        is_valid
    }

    fn save_form(&mut self) -> bool {
        // Validate form fields
        let is_valid = self.validate_form();

        if is_valid {
            self.props
                .edit_investment
                .emit(self.props.investment.clone());
            true
        } else {
            // If the form is not valid, return false
            false
        }
    }
}
