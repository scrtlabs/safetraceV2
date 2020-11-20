import React from "react";
import ReactDOM from "react-dom";
import { compose, withProps } from "recompose";
import {
    withScriptjs,
    withGoogleMap,
    GoogleMap,
    Marker,
    Circle
} from "react-google-maps";

function toColor(num) {

    switch (true) {
        case (num > 0.9):
            return "#ff0000";
        case (0.9 >= num && num > 0.7):
            return "#f45004";
        case (0.7 >= num && num > 0.5):
            return "#f79c01";
        case (0.5 >= num && num > 0.3):
            return "#fff300";
        case (0.3 >= num && num > 0.1):
            return "#d3ff1e";
        default:
            return "#00FF00"
    }

    // let r = Math.floor(num * 255)
    // let g = Math.floor((1 - num) * 255)
    // let b = 0
    // let res = "#" + r.toString(16).padStart(2, "0") + g.toString(16).padStart(2, "0") + b.toString(16).padStart(2, "0");
    // console.log(res)
    // return res;
}

export const MyMapComponent = compose(
    withProps({
        /**
         * Note: create and replace your own key in the Google console.
         * https://console.developers.google.com/apis/dashboard
         * The key "AIzaSyBkNaAGLEVq0YLQMi-PYEMabFeREadYe1Q" can be ONLY used in this sandbox (no forked).
         */
        googleMapURL:
            "https://maps.googleapis.com/maps/api/js?key=<YOUR_KEY_HERE>&v=3.exp&libraries=geometry,drawing,places",
        loadingElement: <div style={{ height: `100%` }} />,
        containerElement: <div style={{ height: `400px` }} />,
        mapElement: <div style={{ height: `100%` }} />,
    }),
    withScriptjs,
    withGoogleMap
)(props => (
    <GoogleMap defaultZoom={11} defaultCenter={{ lat: 52.5200, lng: 13.4050 }}>
        {props.isMarkerShown ?
            props.geoHashes.map((value, index) => (
                //console.log(value)
                //<Marker position={{lat: value.latitude, lng:value.longitude}} />
                <Circle center={{lat: value.pos.latitude, lng:value.pos.longitude}}
                        key={index}
                        radius={305}
                        options={{
                            strokeWidth: 0.01,
                            //strokeColor: `#3b3e00`,
                            strokeColor: toColor(value.power),
                            fillOpacity: 0.2,
                            fillColor: toColor(value.power)
                        }}
                />
            ))

            :
            null
        }
    </GoogleMap>

));

// ReactDOM.render(<MyMapComponent isMarkerShown />, document.getElementById("root"));
