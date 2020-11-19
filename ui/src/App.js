import logo from './logo.svg';
import './App.css';
import {MyMapComponent} from './Maps';
import geohash from 'ngeohash';
import {useEffect, useState} from "react";
import axios from 'axios';
import {EnigmaUtils, CosmWasmClient} from 'secretjs';


function App() {
    const [data, setData] = useState(undefined);

    useEffect(() => {
        // Update the document title using the browser API
        const fetchData = async() => {
            const seed = EnigmaUtils.GenerateNewSeed();
            const client = new CosmWasmClient("http://localhost", seed);

            const codeResult = await client.getCodes();
            console.log(codeResult)
            const code = codeResult[codeResult.length - 1].id;

            const contractResult = await client.getContracts(code);

            //const contractAddr = "secret1tndcaqxkpc5ce9qee5ggqf430mr2z3pedc68dx"
            const contractAddr = contractResult[contractResult.length - 1].address;
            console.log(contractAddr);
            let result = await client.queryContractSmart(contractAddr, {"hot_spot": {"accuracy": 7}});

            console.log(result);
            // let decoded = geoHashes.map((h) => { return {pos: geohash.decode(h.pos), power: h.power / maxPower} });

            let geoHashes = result.hot_spot_response.hot_spots

            let maxPower = geoHashes[0].power
            let minPower = geoHashes[9].power
            console.log(maxPower)
            let decoded = geoHashes.map((h) => { return {pos: geohash.decode(h.geo_location), power: (h.power - minPower) / (maxPower - minPower)} });

            console.log(decoded)

            setData(decoded);
        }

        fetchData();
    }, []);

    // [KeyVal("sv8wrxf", 170296), KeyVal("sv8wrvb", 18200), KeyVal("sv8wx2t", 7336), KeyVal("sv8wrrz", 1120), KeyVal("sv8wx99", 1008), KeyVal("sv8wrxc", 952), KeyVal("sv8wrxb", 840), KeyVal("sv8wrrx", 672), KeyVal("sv8wrwc", 560), KeyVal("sv8wrry", 504)]


  return (
    <div className="App">
      <MyMapComponent isMarkerShown={data !== undefined} geoHashes={data}/>
    </div>
  );
}

export default App;
