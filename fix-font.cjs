const fs = require('fs');
const path = require('path');

const cssPath = path.join(__dirname, 'dist', 'assets');
const cssFiles = fs.readdirSync(cssPath).filter(f => f.endsWith('.css'));

console.log('Fixing Chinese fonts in CSS files...');

cssFiles.forEach(file => {
    const filePath = path.join(cssPath, file);
    let content = fs.readFileSync(filePath, 'utf8');
    
    // Replace Tailwind's default :root font-family
    content = content.replace(
        /:root\{font-family:Inter,Avenir,Helvetica,Arial,sans-serif;/,
        ':root{font-family:-apple-system,BlinkMacSystemFont,"Segoe UI","Microsoft YaHei",Arial,"Helvetica Neue",sans-serif;'
    );
    
    // Also replace any other instances
    content = content.replace(
        /font-family:Inter,Avenir,Helvetica,Arial,sans-serif/g,
        'font-family:-apple-system,BlinkMacSystemFont,"Segoe UI","Microsoft YaHei",Arial,"Helvetica Neue",sans-serif'
    );
    
    // Write back with UTF-8 encoding
    fs.writeFileSync(filePath, content, 'utf8');
    console.log(`  Fixed: ${file}`);
});

console.log('Done! Chinese fonts should now work correctly.');
