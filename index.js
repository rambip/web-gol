
import init from './wasm/game_of_life.js';
import {Universe} from './wasm/game_of_life.js';

// init canvas size
const width = Math.round(document.documentElement.clientWidth/2-20);
const height = Math.round(document.documentElement.clientHeight/2-100);

// init canvas and draw utilities
const canvas = document.getElementById("canvas"); canvas.width = width*2; canvas.height=height*2;
const ctx = canvas.getContext('2d');

const birth_rule = document.getElementById("birth").children;
const survive_rule = document.getElementById("survive").children;

document.getElementById("change").onclick = new_universe;
document.getElementById("random").onclick = random_rule;

let demo;

function generate_random_grid(universe) {
        for (var y = height/4; y < height*3/4; y++) {
                for (var x = width/4; x < width*3/4; x++) {
                        if (Math.random() > .6) {
                                universe.add_cell(x, y);
                        }
                }
        }
}

function new_universe(){
        let birth = ""; let survive = "";
        for (let i = 0; i < 9; i++) {
                if (birth_rule[i].checked) birth += i.toString();
                if (survive_rule[i].checked) survive += i.toString();
        }

        demo = new Universe(birth, survive, width, height, ctx);

        generate_random_grid(demo);
}

function random_rule(){
        for (let i = 0; i < 9; i++) {
                birth_rule[i].checked = Math.random()>0.8;
                survive_rule[i].checked = Math.random()>0.7;
        }
}

function frame_loop(x){
        demo.step();
        demo.draw();
        setTimeout(() => requestAnimationFrame(frame_loop), 20);
}

function run(){
        // run default game of life
        demo = new Universe("3", "23", width, height, ctx);
        generate_random_grid(demo);

        frame_loop()
}

init().then(run);
