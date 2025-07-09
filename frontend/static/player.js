document.addEventListener("DOMContentLoaded", function() {
    const video = document.getElementById("video");
    if (Hls.isSupported()) {
        const hls = new Hls();
        hls.loadSource("http://localhost:3000/media/output.m3u8");
        hls.attachMedia(video);
    }else if(video.canPlayType("application/vnd.apple.mpegurl")) {
        video.src = "http://localhost:3000/media/output.m3u8";
    }
      
})