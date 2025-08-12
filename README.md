<div>

  <h1><code>turbo-webp</code></h1>

<strong>Ultra-fast WebP encoder and decoder, based on Rust's image-webp implementation</strong>

### Web Worker Demo

##### getPixels.ts

```ts
let wasmModuleArrayBuffer: ArrayBuffer;

export async function getPixels(imageBuffer: ArrayBuffer | string): Promise<{
    data: ImageData;
    width: number;
    height: number;
}> {
    return new Promise(async (resolve, reject) => {
        const worker = new Worker(new URL('./getPixelsWorker.ts', import.meta.url));

        worker.onmessage = async (event) => {
            resolve(event.data);
            worker.terminate();
        };

        worker.onerror = (error) => {
            reject(error);
            worker.terminate();
        };

        if (!wasmModuleArrayBuffer) {
            const wasmModuleResponse = await fetch(
                new URL('turbo-webp/turbo_webp_bg.wasm', import.meta.url),
            );
            wasmModuleArrayBuffer = await wasmModuleResponse.arrayBuffer();
        }
        worker.postMessage({ imageBuffer, wasmModuleArrayBuffer });
    });
}

```

##### getPixelsWorker.ts

```ts
import { decode, initSync } from 'turbo-webp';

self.onmessage = async (
    event: MessageEvent<{
        wasmModuleArrayBuffer: ArrayBuffer;
        imageBuffer: ArrayBuffer;
    }>,
) => {
    const { imageBuffer, wasmModuleArrayBuffer } = event.data;

    initSync({
        module: wasmModuleArrayBuffer,
    });

    // 后 8 位包含图像的宽高
    const imageData = decode(new Uint8Array(imageBuffer));

    const dataView = new DataView(imageData.buffer, imageData.byteLength - 8);
    const imageWidth = dataView.getUint32(0, true);
    const imageHeight = dataView.getUint32(4, true);

    self.postMessage({
        data: new ImageData(
            imageData.subarray(0, imageWidth * imageHeight * 4),
            imageWidth,
            imageHeight,
        ),
        width: imageWidth,
        height: imageHeight,
    });
};

```