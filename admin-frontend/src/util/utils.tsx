export const updateSlides = (setSlides: (slides: any) => void) => {
    fetch(`${import.meta.env.VITE_API_BASE_URL}/api/screen/slides`, {
            method: 'GET',
            credentials: 'include'
        })
            .then(response => response.json())
            .then(json => setSlides(json))
            .catch(error => console.error('Error fetching slides:', error));
// setSlides([{ id: 'dummy-id', caption: 'Dummy Slide', start_date: new Date(), end_date: new Date(), active: true, filetype: 'image/png' }, { id: 'dummy-id2', caption: 'Dummy Slide', start_date: new Date(), end_date: new Date(), active: true, filetype: 'image/png' }, { id: 'dummy-id3', caption: 'Dummy Slide', start_date: new Date(), end_date: new Date(), active: true, filetype: 'image/png' }]);
}