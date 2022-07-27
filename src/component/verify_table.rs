use yew::prelude::*;

use crate::dto::verify_data::VerifyDataDto;

#[derive(Clone, Properties, PartialEq)]
pub struct VerifyTableProps {
    pub verify_data: Option<VerifyDataDto>
}

#[function_component(VerifyTable)]
pub fn verify_table(VerifyTableProps { verify_data }: &VerifyTableProps) -> Html {
    match verify_data {
        Some(verify_data) => html! {
            <div>
                <table>
                    if verify_data.data.is_some(){
                        <tr>
                            <th>{"Token_Id"}</th>
                            <td>{format!("{}", verify_data.data.clone().unwrap().id)}</td>
                        </tr>
                        <tr>
                            <th>{"Data"}</th>
                            <td>{format!("{}", verify_data.data.clone().unwrap().data)}</td>
                        </tr>
                        <tr>
                            <th>{"Merkle Root"}</th>
                            <td>{format!("{}", verify_data.data.clone().unwrap().merkle_root)}</td>
                        </tr>
                        <tr>
                            <th>{"Tx Hash"}</th>
                            <td>{format!("{}", verify_data.data.clone().unwrap().tx_hash)}</td>
                        </tr>
                        <tr>
                            <th>{"Block Number"}</th>
                            <td>{format!("{}", verify_data.data.clone().unwrap().block_number)}</td>
                        </tr>
                        <tr>
                            <th>{"Timestamp"}</th>
                            <td>{format!("{}", verify_data.data.clone().unwrap().timestamp)}</td>
                        </tr>
                        <tr>
                            <th>{"TSA Gen Time"}</th>
                            <td>{format!("{}", verify_data.data.clone().unwrap().tsa_gen_time)}</td>
                        </tr>
                        
                    }
                </table>
            </div>
        },
        None => html! {
            <div>
                <table>
                <tr>
                    <th>{"Token_Id"}</th>
                </tr>
                <tr>
                    <th>{"Data"}</th>
                </tr>
                <tr>
                    <th>{"Merkle Root"}</th>
                </tr>
                <tr>
                    <th>{"Tx Hash"}</th>
                </tr>
                <tr>
                    <th>{"Block Number"}</th>
                </tr>
                <tr>
                    <th>{"Timestamp"}</th>
                </tr>
                <tr>
                    <th>{"TSA Gen Time"}</th>
                </tr>
                </table>
            </div>
        },
    }

    
}
