use std::borrow::Borrow;

use crate::types;
use reqwest::Error;

pub async fn get_raw_buses() -> Result<Vec<types::RawBus>, Error> {
    let res: types::Response = serde_json::from_str(
        &reqwest::get("https://datos.santander.es/api/rest/datasets/lineas_bus.json")
            .await
            .unwrap()
            .text()
            .await
            .unwrap(),
    )
    .unwrap();
    Ok(res.resources)
}

pub async fn get_raw_bus(number: String) -> Result<Option<types::RawBus>, Error> {
    let res: types::Response = serde_json::from_str(
        &reqwest::get(
            "https://datos.santander.es/api/rest/datasets/lineas_bus/".to_owned()
                + &number
                + ".json",
        )
        .await
        .unwrap()
        .text()
        .await
        .unwrap(),
    )
    .unwrap();

    let xd = res.clone().resources;
    if res.resources.clone().into_iter().count() > 0 {
        Ok(Some(xd.clone()[0]))
    } else {
        Ok(None)
    }
}

pub async fn get_buses() -> Result<Vec<types::Bus>, Error> {
    Ok(get_raw_buses()
        .await
        .unwrap()
        .into_iter()
        .map(|raw_bus| {
            let destinations_vector = raw_bus
                .name
                .split('-')
                .map(|s| s.to_string())
                .collect::<Vec<String>>();
            let destinations: types::Destinations =
                match destinations_vector.clone().into_iter().count() == 1 {
                    true => {
                        let destinations_vector: Vec<String> = destinations_vector[0]
                            .split("/")
                            .map(|s| s.into())
                            .collect();
                        types::Destinations {
                            a: destinations_vector.clone(),
                            b: None,
                        }
                    }
                    false => types::Destinations {
                        a: destinations_vector[0]
                            .split("/")
                            .map(|s| s.into())
                            .collect(),
                        b: Some(
                            destinations_vector[1]
                                .split("/")
                                .map(|s| s.into())
                                .collect(),
                        ),
                    },
                };

            types::Bus {
                number: raw_bus.number,
                id: raw_bus.id.parse().unwrap(),
                destinations,
            }
        })
        .collect::<Vec<types::Bus>>())
}

pub async fn get_bus(number: String) -> Result<Option<types::Bus>, Error> {
    let mut bus: Option<types::Bus> = None;
    for bus_read in get_buses().await.unwrap() {
        if bus_read.number == number {
            bus = Some(bus_read);
        }
    }
    Ok(bus)
}
