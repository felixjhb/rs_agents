use std::{future::Future, io::ErrorKind};

use image::{ImageBuffer, Rgb};
use tokio::task;
use crate::agent::Agent;

pub struct Map<const WIDTH: usize, const HEIGHT: usize, const NUMBER_OF_AGENTS: usize> {
    value_map: [[u32; HEIGHT]; WIDTH],
    agents: [Agent; NUMBER_OF_AGENTS],
    dispersion_rate: u32,
    dispersion_radius: u32,
}

impl<const WIDTH: usize, const HEIGHT: usize, const NUMBER_OF_AGENTS: usize> Map<WIDTH, HEIGHT, NUMBER_OF_AGENTS> {
    pub fn new(dispersion_rate: u32, dispersion_radius: u32) -> Self {
        Map {
            value_map: [[0; HEIGHT]; WIDTH],
            agents: [Agent::new(); NUMBER_OF_AGENTS],
            dispersion_rate,
            dispersion_radius,
        }
    }

    pub fn poll(xpos: usize, ypos: usize, angle: f64, poll_distance: f64) -> usize {
        //TODO
        1
    }

    fn add_pheromones(&mut self) {
        //TODO
    }

    async fn update_batch_agents(batch: &[Agent]) -> Result<(), ErrorKind> {
        Ok(())
    }

    async fn update_agents(&self) {
        const BATCH_SIZE: usize = 1024;
        for batch in self.agents.chunks(BATCH_SIZE) {
            task::spawn(async {Map::<WIDTH, HEIGHT, NUMBER_OF_AGENTS>::update_batch_agents(&batch).await});
        }
    }

    fn disperse_value_map(&mut self) {
        //TODO
    }

    pub async fn update(&mut self) {
        self.disperse_value_map();
        self.update_agents().await;
        self.add_pheromones();
    }

    pub fn render(&self) -> ImageBuffer<Rgb<u8>,Vec<u8>> {
        let mut my_image = ImageBuffer::<Rgb<u8>, Vec<u8>>::new(WIDTH, HEIGHT);
        my_image
    }
}