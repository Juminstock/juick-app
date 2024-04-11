import styles from './Hero.module.css';

export const Hero = () => {
    return(
        <section className={styles.Hero}>
            <h1>
                Im the hero!
            </h1>
            <h2>
                Empowering your tomorrow, today!
            </h2>
        </section>        
    );
}