import { SlideData } from "../../types/slides/SlideData";
import Slideshow from "../slides/Slideshow"

function FullscreenSlideshowLayout({slides}: {slides: SlideData[]}) {
  return <>
    <Slideshow slides={slides} />
  </>;
}

export default FullscreenSlideshowLayout;
