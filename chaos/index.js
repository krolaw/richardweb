import('./pkg')
    .then(wasm => {
        const canvas = document.getElementById('drawing');
        const ctx = canvas.getContext('2d');

        const pointsInput = document.getElementById('points');
        const ratioInput = document.getElementById('ratio');
        const rotateInput = document.getElementById('rotate');
        const renderBtn = document.getElementById('render');

        const preset = document.getElementById('preset');

        render = () => {
            const points = parseInt(pointsInput.value,10) || 0;
            const ratio = parseFloat(ratioInput.value) || 0.0;
            const rotate = parseFloat(rotateInput.value) || 0.0;
            wasm.draw(ctx, 600, 600, points, ratio, rotate);
        };

        renderBtn.addEventListener('click', render);

        preset.addEventListener('change', (event) => {
            let z = event.target.value.split(",");
            pointsInput.value = z[0];
            ratioInput.value = z[1];
            rotateInput.value = z.length >2 ? z[2] : 0 ;
            render();
        });

        wasm.draw(ctx, 600, 600, 3, 0.5, 0.0);
    })
    .catch(console.error);
