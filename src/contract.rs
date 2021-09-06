
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint64, to_binary};

use crate::error::ContractError;
use crate::msg::{CountResponse, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::PING_COUNT;
// use crate::state::{State, STATE};
use std::ops::Add;
use cosmwasm_std::StdError;
use cosmwasm_std::attr;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    PING_COUNT.save(deps.storage, &Uint64::zero())?;
    let res = Response::new();
    Ok(res)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    let mut count = Uint64::zero();
    match msg {
        ExecuteMsg::Ping {} => {
            let _increment: Result<Uint64, StdError> = PING_COUNT.update(deps.storage, |exists| {
                count = exists.add(Uint64::from(1u8));
                Ok(count)
            });
            let mut res = Response::new();
            res.attributes.push(attr("ping_count", count));
            // res.attributes.push(attr("pong", "pong"));
            res.data = Some(to_binary("pong")?);
            Ok(res)
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    let count = PING_COUNT.load(deps.storage)?;
    to_binary(&count)
}

#[allow(dead_code)]
fn query_count(_deps: Deps) -> StdResult<CountResponse> {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary};


    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies(&[]);

        let msg = InstantiateMsg {};
        let info = mock_info("creator", &[]);

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // it worked, let's query the state
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let value: Uint64 = from_binary(&res).unwrap();
        assert_eq!(Uint64::zero(), value);
    }

    #[test]
    fn test_ping() {
        let mut deps = mock_dependencies(&[]);

        let msg = InstantiateMsg {};
        let info = mock_info("creator", &[]);

        instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

        let msg = ExecuteMsg::Ping {};

        let res = execute(deps.as_mut(), mock_env(), info.clone(), msg.clone()).unwrap();
        assert_eq!(res.attributes.len(), 1);
        assert_eq!(res.attributes, vec![attr("ping_count", 1.to_string())]);
        let data: String = from_binary(&res.data.unwrap()).unwrap();
        assert_eq!(data, "pong");

        let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(res.attributes.len(), 1);
        assert_eq!(res.attributes, vec![attr("ping_count", 2.to_string())]);
        let data: String = from_binary(&res.data.unwrap()).unwrap();
        assert_eq!(data, "pong");
    }

    /*
    #[test]
    fn increment() {
        let mut deps = mock_dependencies(&coins(2, "token"));

        let msg = InstantiateMsg { count: 17 };
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // beneficiary can release it
        let info = mock_info("anyone", &coins(2, "token"));
        let msg = ExecuteMsg::Increment {};
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        // should increase counter by 1
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let value: CountResponse = from_binary(&res).unwrap();
        assert_eq!(18, value.count);
    }

    #[test]
    fn reset() {
        let mut deps = mock_dependencies(&coins(2, "token"));

        let msg = InstantiateMsg { count: 17 };
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // beneficiary can release it
        let unauth_info = mock_info("anyone", &coins(2, "token"));
        let msg = ExecuteMsg::Reset { count: 5 };
        let res = execute(deps.as_mut(), mock_env(), unauth_info, msg);
        match res {
            Err(ContractError::Unauthorized {}) => {}
            _ => panic!("Must return unauthorized error"),
        }

        // only the original creator can reset the counter
        let auth_info = mock_info("creator", &coins(2, "token"));
        let msg = ExecuteMsg::Reset { count: 5 };
        let _res = execute(deps.as_mut(), mock_env(), auth_info, msg).unwrap();

        // should now be 5
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let value: CountResponse = from_binary(&res).unwrap();
        assert_eq!(5, value.count);
    }
    */
}
