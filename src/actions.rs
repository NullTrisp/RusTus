use crate::types;
use reqwest::Error;

async fn get_raw_stops(offset: types::Offest) -> Result<Vec<types::RawStop>, Error> {
    let res: types::ResponseStops = serde_json::from_str(
        &reqwest::get(
            "https://datos.santander.es/api/rest/datasets/paradas_bus.json?items=".to_owned()
                + &offset.to.to_string(),
        )
        .await
        .unwrap()
        .text()
        .await
        .unwrap(),
    )
    .unwrap();
    Ok(res.resources)
}

async fn get_raw_stop(id: String) -> Result<Option<types::RawStop>, Error> {
    let res: types::ResponseStops = serde_json::from_str(
        &reqwest::get(
            "http://datos.santander.es/api/datos/paradas_bus/".to_owned() + &id + ".json",
        )
        .await
        .unwrap()
        .text()
        .await
        .unwrap(),
    )
    .unwrap();

    match res.resources.get(0) {
        Some(stop) => Ok(Some(stop.clone())),
        None => Ok(None),
    }
}

async fn get_raw_buses() -> Result<Vec<types::RawBus>, Error> {
    let res: types::ResponseBuses = serde_json::from_str(
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

async fn get_raw_bus(number: String) -> Result<Option<types::RawBus>, Error> {
    let res: types::ResponseBuses = serde_json::from_str(
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

    match res.resources.get(0) {
        Some(bus) => Ok(Some(bus.clone())),
        None => Ok(None),
    }
}

fn get_destinations(destionation: String) -> types::Destinations {
    let destinations_vector = destionation
        .split('-')
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
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
    }
}

pub async fn get_buses() -> Result<Vec<types::Bus>, Error> {
    Ok(get_raw_buses()
        .await
        .unwrap()
        .into_iter()
        .map(|raw_bus| types::Bus {
            number: raw_bus.number,
            id: raw_bus.id.parse().unwrap(),
            destinations: get_destinations(raw_bus.name),
        })
        .collect::<Vec<types::Bus>>())
}

pub async fn get_bus(bus_number: String) -> Result<Option<types::Bus>, Error> {
    match get_raw_bus(bus_number).await {
        Ok(bus_found) => match bus_found {
            Some(bus) => Ok(Some(types::Bus {
                number: bus.number,
                destinations: get_destinations(bus.name),
                id: bus.id.parse().unwrap(),
            })),
            None => Ok(None),
        },
        Err(err) => Err(err),
    }
}

pub async fn get_stops(offset: types::Offest) -> Result<Vec<types::Stop>, Error> {
    Ok(get_raw_stops(offset)
        .await
        .unwrap()
        .into_iter()
        .map(|raw_stop| types::Stop {
            id: raw_stop.id.parse().unwrap(),
            direction: raw_stop.direction,
            name: raw_stop.name,
            latitude: raw_stop.wgs84_pos_lat.parse().unwrap(),
            longitude: raw_stop.wgs84_pos_long.parse().unwrap(),
            number: raw_stop.number.parse().unwrap(),
        })
        .collect())
}

pub async fn get_stop(id: String) -> Result<Option<types::Stop>, Error> {
    match get_raw_stop(id).await {
        Ok(stop) => match stop {
            Some(raw_stop) => Ok(Some(types::Stop {
                id: raw_stop.id.parse().unwrap(),
                direction: raw_stop.direction,
                name: raw_stop.name,
                latitude: raw_stop.wgs84_pos_lat.parse().unwrap(),
                longitude: raw_stop.wgs84_pos_long.parse().unwrap(),
                number: raw_stop.number.parse().unwrap(),
            })),
            None => Ok(None),
        },
        Err(err) => Err(err),
    }
}
