import React from 'react';

const NewSlide: React.FC = () => {
    const handleSubmit = (event: React.FormEvent<HTMLFormElement>) => {
        event.preventDefault();
        const data = new FormData(event.currentTarget);

        // Visible is an empty string if the checkbox is not checked; we want it to be false instead
        data.set('visible', data.get('visible') || 'false');

        fetch('http://localhost:8080/api/screen/slides/save', {
            method: 'POST',
            body: data
        }).then(response => {
            if (response.ok) {
                console.log('Slide uploaded');
                //TODO update slides when a new slide is added
            } else {
                console.log('Failed to upload slide');
            }
        });
    }
    return (
        <>
            <h1>New Slide</h1>
            <form onSubmit={(event) => handleSubmit(event)}>
                <button type='submit'>Submit</button>
                <label htmlFor='caption'>Caption</label>
                <input type='text' id='caption' name='caption' />
                <label htmlFor='startDate'>Start Date</label>
                <input type='date' id='startDate' name='start' />
                <label htmlFor='endDate'>End Date</label>
                <input type='date' id='endDate' name='end' />
                <label htmlFor='active'>Active</label>
                <input type='checkbox' id='active' name='visible' value = 'true'/>
                <label htmlFor='file'>File</label>
                <input type='file' id='file' name='imageFile' />

            </form>
        </>
    );
};

export default NewSlide;